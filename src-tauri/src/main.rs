// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::fs;
use std::io::{BufRead, BufReader};
use std::process::Command;
use serde::{Serialize, Deserialize};



// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Group not found")]
    GroupNotFound,
    #[error("Permission denied: Administrator privileges required")]
    PermissionDenied,
    #[error("Authentication failed")]
    AuthenticationFailed,
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// 表示一个hosts文件条目
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HostsEntry {
    pub ip: String,
    pub hostname: String,
    pub comment: Option<String>,
}

// 分组标记的前缀和后缀
const GROUP_MARKER_PREFIX: &str = "#------- ";
const GROUP_MARKER_SUFFIX: &str = " -------";

// 获取默认hosts文件路径（跨平台）
fn get_hosts_path() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        r"C:\Windows\System32\drivers\etc\hosts"
    }
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        "/etc/hosts"
    }
}

// 检查是否有权限读写hosts文件
#[tauri::command]
fn check_hosts_permission() -> Result<bool, Error> {
    let hosts_path = get_hosts_path();
    
    // 尝试读取文件
    match fs::read_to_string(hosts_path) {
        Ok(_) => {
            // 尝试以追加模式打开文件来检查写权限
            match fs::OpenOptions::new().append(true).open(hosts_path) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        },
        Err(_) => Ok(false),
    }
}

// 使用管理员权限写入hosts文件 (macOS/Linux)
#[cfg(any(target_os = "macos", target_os = "linux"))]
fn write_hosts_with_sudo(content: &str) -> Result<(), Error> {
    // 创建临时文件
    let temp_path = "/tmp/hosts_temp";
    fs::write(temp_path, content)?;
    
    #[cfg(target_os = "macos")]
    {
        // 在 macOS 上使用 osascript 来触发图形化的权限提升对话框
        let script = format!(
            r#"do shell script "cp {} {}" with administrator privileges"#,
            temp_path,
            get_hosts_path()
        );
        
        let output = Command::new("osascript")
            .args(&["-e", &script])
            .output();
        
        // 删除临时文件
        let _ = fs::remove_file(temp_path);
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    if stderr.contains("User canceled") || stderr.contains("cancelled") {
                        Err(Error::AuthenticationFailed)
                    } else {
                        Err(Error::PermissionDenied)
                    }
                }
            },
            Err(_) => Err(Error::PermissionDenied),
        }
    }
    
    #[cfg(all(target_os = "linux", not(target_os = "macos")))]
    {
        // Linux 上仍然使用 sudo 命令
        let output = Command::new("sudo")
            .args(&["cp", temp_path, get_hosts_path()])
            .output();
        
        // 删除临时文件
        let _ = fs::remove_file(temp_path);
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(())
                } else {
                    Err(Error::AuthenticationFailed)
                }
            },
            Err(_) => Err(Error::PermissionDenied),
        }
    }
}

// Windows版本的权限写入（如果需要的话）
#[cfg(target_os = "windows")]
fn write_hosts_with_elevation(content: &str) -> Result<(), Error> {
    // Windows上直接尝试写入，如果失败则返回权限错误
    match fs::write(get_hosts_path(), content) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::PermissionDenied),
    }
}

// 安全的hosts文件写入函数
fn safe_write_hosts(content: &str) -> Result<(), Error> {
    // 首先尝试直接写入
    match fs::write(get_hosts_path(), content) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                // 权限不足，尝试提权写入
                #[cfg(any(target_os = "macos", target_os = "linux"))]
                {
                    write_hosts_with_sudo(content)
                }
                #[cfg(target_os = "windows")]
                {
                    write_hosts_with_elevation(content)
                }
            } else {
                Err(Error::Io(e))
            }
        }
    }
}

// 获取所有分组的名称
#[tauri::command]
fn get_all_hosts_groups() -> Result<Vec<String>, Error> {
    let content = fs::read_to_string(get_hosts_path())?;
    let mut groups = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with(GROUP_MARKER_PREFIX) && line.ends_with(GROUP_MARKER_SUFFIX) {
            let group_name = &line[GROUP_MARKER_PREFIX.len()..line.len() - GROUP_MARKER_SUFFIX.len()];
            
            if !group_name.ends_with(" end") {
                groups.push(group_name.to_string());
            }
        }
    }
    
    Ok(groups)
}

// 获取指定分组的所有条目
#[tauri::command]
fn get_hosts_group_entries(group_name: &str) -> Result<Vec<HostsEntry>, Error> {
    let content = fs::read_to_string(get_hosts_path())?;
    let mut entries = Vec::new();
    let start_marker = format!("{}{}{}", GROUP_MARKER_PREFIX, group_name, GROUP_MARKER_SUFFIX);
    let end_marker = format!("{}{} end{}", GROUP_MARKER_PREFIX, group_name, GROUP_MARKER_SUFFIX);
    
    let mut in_group = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        if line == start_marker {
            in_group = true;
            continue;
        }
        
        if line == end_marker {
            in_group = false;
            continue;
        }
        
        if in_group && !line.is_empty() && !line.starts_with('#') {
            let parts: Vec<&str> = line.splitn(3, |c| c == ' ' || c == '\t').collect();
            if parts.len() < 2 {
                continue;
            }
            
            let ip = parts[0].trim().to_string();
            let hostname = parts[1].trim().to_string();
            let comment = parts.get(2).map(|c| c.trim_start_matches('#').trim().to_string());
            
            entries.push(HostsEntry { ip, hostname, comment });
        }
    }
    
    Ok(entries)
}

// 添加带分组的hosts片段（如果分组已存在则替换）
#[tauri::command]
fn add_hosts_fragment_with_group(group_name: &str, fragment: &str) -> Result<(), Error> {
    // 首先尝试删除已存在的分组（如果存在的话）
    let _ = remove_hosts_group(group_name); // 忽略错误，因为分组可能不存在
    
    // 读取当前hosts文件内容
    let current_content = fs::read_to_string(get_hosts_path())?;
    let mut new_content = current_content;
    
    // 添加开始标记
    new_content.push_str(&format!("{}{}{}\n", GROUP_MARKER_PREFIX, group_name, GROUP_MARKER_SUFFIX));
    
    // 添加片段内容（过滤掉空行）
    for line in fragment.lines() {
        let line = line.trim();
        if !line.is_empty() {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }
    
    // 添加结束标记
    new_content.push_str(&format!("{}{} end{}\n", GROUP_MARKER_PREFIX, group_name, GROUP_MARKER_SUFFIX));
    
    // 使用安全写入方法
    safe_write_hosts(&new_content)
}

// 删除指定分组的所有条目
#[tauri::command]
fn remove_hosts_group(group_name: &str) -> Result<(), Error> {
    let start_marker = format!("{}{}{}", GROUP_MARKER_PREFIX, group_name, GROUP_MARKER_SUFFIX);
    let end_marker = format!("{}{} end{}", GROUP_MARKER_PREFIX, group_name, GROUP_MARKER_SUFFIX);
    
    let content = fs::read_to_string(get_hosts_path())?;
    let mut new_content = String::new();
    
    let mut in_group = false;
    let mut group_found = false;
    
    for line in content.lines() {
        if line.trim() == start_marker {
            in_group = true;
            group_found = true;
            continue;
        }
        
        if in_group {
            if line.trim() == end_marker {
                in_group = false;
            }
            continue;
        }
        
        // 不在组内的行保留
        new_content.push_str(line);
        new_content.push('\n');
    }
    
    if group_found {
        // 使用安全写入方法
        safe_write_hosts(&new_content)
    } else {
        Err(Error::GroupNotFound)
    }
}

// 原有功能：获取默认hosts内容
#[tauri::command]
fn get_default_hosts() -> Result<String, Error> {
    let hosts = fs::read_to_string(get_hosts_path())?;
    Ok(hosts)
}

// 原有功能：添加内容到特定作用域（兼容旧版本）
#[tauri::command]
fn add_to_hosts_scope(new_content: &str) -> Result<(), Error> {
    let file = fs::File::open(get_hosts_path())?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut scope_exists = false;

    for line in reader.lines() {
        let line = line?;
        if line.contains("# ADD BY HOSTS SWITCHER") {
            scope_exists = true;
        }
        if line.contains("# END OF HOSTS SWITCHER") {
            lines.push(new_content.to_string());
        }

        lines.push(line);
    }
    
    if !scope_exists {
        lines.push("# ADD BY HOSTS SWITCHER".to_string());
        lines.push(new_content.to_string());
        lines.push("# END OF HOSTS SWITCHER".to_string());
    }
    
    let final_content = lines.join("\n");
    safe_write_hosts(&final_content)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_default_hosts,
            add_to_hosts_scope,
            get_all_hosts_groups,
            get_hosts_group_entries,
            add_hosts_fragment_with_group,
            remove_hosts_group,
            check_hosts_permission
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}    