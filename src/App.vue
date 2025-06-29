<template>
  <Suspense>
    <div id="main" class="flex h-screen">
      <!-- 左侧分组列表 -->
      <div class="w-80 p-3 flex flex-col bg-gray-50">
        <div class="header flex items-center text-gray-500">
          <img src="/node_servers.png" alt="Hosts Manager" class="h-11 w-11" />
          <span class="ml-2 font-semibold text-lg"> A Hosts Switch Tool</span>
        </div>
        <div class="border-t border-gray-200 mt-6"></div>
        
        <!-- 分组列表 -->
        <div class="groups mt-4 p-3 flex-grow overflow-y-auto">
          <div class="flex items-center justify-between mb-2">
            <h3 class="text-sm font-medium text-gray-700">分组列表</h3>
            <button 
              class="w-6 h-6 bg-blue-500 text-white rounded-full hover:bg-blue-600 transition-colors flex items-center justify-center text-sm font-bold"
              @click="refreshGroups"
              title="刷新分组列表"
            >
              ↻
            </button>
          </div>
          
          <ul>
            <!-- 预览组 -->
            <li>
              <button
                class="w-full box-border py-1.5 px-3 rounded mt-2 flex justify-between items-center border border-gray-300 bg-gray-100"
                :class="{ 'border-blue-500 bg-blue-50': currentEditingIndex === 0 }"
                @click="showHostsDetail(0)"
              >
                <span class="text-gray-700 font-medium">系统 Hosts</span>
                <span class="text-xs text-gray-500">只读</span>
              </button>
            </li>
            
            <!-- 其他分组 -->
            <template v-for="(item, index) in groups" :key="item.label">
              <li>
                <button
                  class="w-full box-border py-1.5 px-3 rounded mt-2 flex justify-between items-center border"
                  :class="{
                    'border-blue-500 bg-blue-50': currentEditingIndex === index + 1,
                    'border-green-500 bg-green-50': item.active,
                  }"
                  @click="showHostsDetail(index + 1)"
                >
                  <div class="flex items-center">
                    <!-- 编辑模式：内联输入框 -->
                    <input
                      v-if="editingGroupIndex === index"
                      v-model="editingGroupName"
                      class="text-gray-700 mr-2 bg-white border border-blue-400 rounded px-2 py-1 text-sm"
                      :style="{ maxWidth: '12em' }"
                      @blur="finishEditGroupName"
                      @keyup.enter="finishEditGroupName"
                      @keyup.esc="cancelEditGroupName"
                      data-edit-group="true"
                      @click.stop
                    />
                    <!-- 显示模式：可双击编辑的文本 -->
                    <span 
                      v-else
                      class="text-gray-700 mr-2 truncate group-name cursor-pointer hover:bg-gray-100 px-1 py-0.5 rounded transition-colors" 
                      :title="item.label"
                      :style="{ maxWidth: '12em' }"
                      @dblclick.stop="startEditGroupName(index, item.label)"
                    >
                      {{ item.label }}
                    </span>
                  </div>
                  <div class="flex items-center">
                    <button
                      class="btn-action mr-1 text-2xs px-1.5 py-1 rounded-full hover:bg-gray-200"
                      @click.stop="toggleGroupActive(item)"
                    >
                      <span 
                        class="text-xs font-bold"
                        :class="{ 'text-green-600': item.active, 'text-gray-400': !item.active }"
                      >
                        {{ item.active ? 'ON' : 'OFF' }}
                      </span>
                    </button>
                    <!-- 编辑按钮已移除，避免报错
                    <button
                      class="btn-action mr-1 text-2xs px-1.5 py-1 rounded-full hover:bg-gray-200"
                      @click.stop="editItem(index)"
                    >
                      <img src="/edit.png" alt="Edit" width="14" height="14" />
                    </button>
                    -->
                    <button
                      class="btn-action text-2xs px-1.5 py-1 rounded-full hover:bg-gray-200"
                      @click.stop="deleteItem(index)"
                    >
                      <img src="/trash.png" alt="Delete" width="14" height="14" />
                    </button>
                  </div>
                </button>
              </li>
            </template>
            
            <!-- 添加新分组 -->
            <li v-if="isAddingNewGroup">
              <div class="w-full box-border py-1.5 px-3 rounded mt-2 border border-blue-300 bg-blue-50">
                <div class="flex items-center">
                  <input
                    ref="newGroupInput"
                    class="flex-grow bg-transparent text-gray-700 placeholder-gray-500 focus:outline-none text-sm"
                    type="text"
                    v-model="newGroupName"
                    placeholder="输入分组名称..."
                    @keyup.enter="createNewGroup"
                    @keyup.esc="cancelAddGroup"
                    @blur="onNewGroupInputBlur"
                  />
                  <div class="flex items-center space-x-1 ml-2">
                    <button
                      class="btn-action px-2 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                      @click="createNewGroup"
                      title="添加分组"
                    >
                      ✓
                    </button>
                    <button
                      class="btn-action px-2 py-1 text-xs bg-gray-400 text-white rounded hover:bg-gray-500 transition-colors"
                      @click="cancelAddGroup"
                      title="取消"
                    >
                      ✕
                    </button>
                  </div>
                </div>
              </div>
            </li>
          </ul>
        </div>
        
        <!-- 添加分组按钮 -->
        <div class="flex justify-center items-center sticky bottom-0 p-3 bg-white border-t border-gray-200">
          <button
            class="w-8 h-8 bg-blue-500 text-white rounded-full hover:bg-blue-600 transition-colors flex items-center justify-center text-lg font-bold"
            @click="startAddGroup"
            title="添加分组"
          >
            +
          </button>
        </div>
      </div>
      
      <!-- 右侧编辑器 -->
      <div class="flex-1 editor" ref="editorArea">
        <div class="p-4 bg-gray-50 border-b border-gray-200 flex justify-between items-center">
          <h2 class="text-lg font-medium text-gray-800">
            {{ currentGroup?.label || '系统 Hosts' }}
          </h2>
          <div class="flex space-x-2">
            <button
              v-if="currentEditingIndex > 0 && editorContentChanged"
              class="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
              @click="saveCurrentGroup"
            >
              保存
            </button>
            <button
              v-if="currentEditingIndex > 0 && editorContentChanged"
              class="px-3 py-1 text-sm bg-gray-500 text-white rounded hover:bg-gray-600 transition-colors"
              @click="undoChanges"
            >
              撤销
            </button>
          </div>
        </div>
        <div class="editor-container"></div>
      </div>
    </div>
  </Suspense>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, getCurrentInstance, nextTick, computed } from "vue";
import { basicSetup, EditorView } from "codemirror";
import { ViewUpdate } from "@codemirror/view";
import { Compartment } from "@codemirror/state";
import { StreamLanguage } from "@codemirror/language";
import { coffeeScript } from "@codemirror/legacy-modes/mode/coffeescript";
import { invoke } from "@tauri-apps/api/tauri";
import { debounce } from "./utils/common";

interface Group {
  label: string;
  hosts: string;
  active: boolean;
  originalContent?: string; // 用于跟踪变更
}

// 状态管理
const editorArea = ref<Element | null>(null);
const newGroupInput = ref<HTMLInputElement | null>(null);
const editor = ref<EditorView | null>(null);
const currentEditingIndex = ref(0); // 0 表示系统 hosts
const editableCompartment = new Compartment();
const groups = ref<Array<Group>>([]);
const isAddingNewGroup = ref(false);
const newGroupName = ref("");
const editorContentChanged = ref(false);
const internalInstance = getCurrentInstance();
const systemHosts = ref<string>("");
const editingGroupIndex = ref(-1); // 正在编辑名称的分组索引
const editingGroupName = ref(""); // 编辑中的分组名称

// 安全的消息显示函数
const showMessage = (message: string, type: 'success' | 'error' | 'warning' | 'info' = 'info') => {
  const messageApi = internalInstance?.appContext.config.globalProperties.$message;
  if (messageApi && messageApi[type]) {
    messageApi[type](message);
  } else {
    // 降级到控制台输出
    console.log(`[${type.toUpperCase()}] ${message}`);
    // 或者使用原生alert作为降级方案
    if (type === 'error') {
      alert(`错误: ${message}`);
    } else if (type === 'warning') {
      alert(`警告: ${message}`);
    } else {
      alert(message);
    }
  }
};

// 计算属性
const currentGroup = computed(() => {
  if (currentEditingIndex.value === 0) return null;
  return groups.value[currentEditingIndex.value - 1];
});

// 刷新分组列表
const refreshGroups = async () => {
  try {
    // 获取本地存储的分组
    const localGroupsStr = localStorage.getItem("hosts-groups");
    const localGroups: Group[] = localGroupsStr ? JSON.parse(localGroupsStr) : [];
    
    // 获取 hosts 文件中的分组
    const groupNames: string[] = await invoke("get_all_hosts_groups");
    const hostsGroups = await Promise.all(
      groupNames.map(async (name) => {
        const entries: any[] = await invoke("get_hosts_group_entries", { groupName: name });
        const hosts = entries.map((entry: any) => 
          `${entry.ip}\t${entry.hostname}${entry.comment ? `\t# ${entry.comment}` : ""}`
        ).join("\n");
        return {
          label: name,
          hosts,
          active: false,
          originalContent: hosts
        };
      })
    );
    
    // 合并分组：创建一个 Map 来去重并合并数据
    const mergedGroupsMap = new Map<string, Group>();
    
    // 先添加本地存储的分组
    localGroups.forEach(group => {
      mergedGroupsMap.set(group.label, {
        label: group.label,
        hosts: group.hosts,
        active: false, // 初始设为 false，稍后检查激活状态
        originalContent: group.hosts
      });
    });
    
    // 然后添加或更新 hosts 文件中的分组
    hostsGroups.forEach(group => {
      if (mergedGroupsMap.has(group.label)) {
        // 如果本地有该分组，更新 hosts 内容（以 hosts 文件为准）
        const existingGroup = mergedGroupsMap.get(group.label)!;
        existingGroup.hosts = group.hosts;
        existingGroup.originalContent = group.hosts;
      } else {
        // 如果本地没有该分组，直接添加
        mergedGroupsMap.set(group.label, group);
      }
    });
    
    // 转换为数组
    const mergedGroups = Array.from(mergedGroupsMap.values());
    groups.value = mergedGroups;
    
    // 检查激活状态
    await checkActiveGroups();
    
    // 如果当前正在编辑的分组不存在了，切换到系统 hosts
    if (currentEditingIndex.value > groups.value.length) {
      showHostsDetail(0);
    }
    
    return mergedGroups;
  } catch (error) {
    console.error("刷新分组失败:", error);
    showMessage("刷新分组失败", "error");
    
    // 如果刷新失败，至少加载本地存储的分组
    const localGroupsStr = localStorage.getItem("hosts-groups");
    if (localGroupsStr) {
      try {
        const localGroups: Group[] = JSON.parse(localGroupsStr);
        groups.value = localGroups;
        return localGroups;
      } catch (parseError) {
        console.error("解析本地分组失败:", parseError);
      }
    }
    
    return [];
  }
};

// 检查哪些分组是激活的
const checkActiveGroups = async () => {
  try {
    const sysHosts: string = await invoke("get_default_hosts") as string;
    systemHosts.value = sysHosts;
    
    // 获取当前 hosts 文件中实际存在的分组
    const activeGroupNames: string[] = await invoke("get_all_hosts_groups");
    
    for (const group of groups.value) {
      // 检查分组是否在 hosts 文件中存在（更精确的检查）
      group.active = activeGroupNames.includes(group.label);
    }
  } catch (error) {
    console.error("检查激活状态失败:", error);
    // 如果检查失败，使用简化的检查方式
    for (const group of groups.value) {
      group.active = systemHosts.value.includes(group.hosts);
    }
  }
};

// 显示分组详情
const showHostsDetail = (index: number) => {
  if (currentEditingIndex.value === index) return;
  if (editorContentChanged.value && currentEditingIndex.value > 0) {
    const dialogApi = internalInstance?.appContext.config.globalProperties.$dialog;
    if (dialogApi) {
      dialogApi({
        title: "保存更改",
        message: "是否保存当前分组的更改？",
        confirmText: "保存",
        cancelText: "取消",
        showCancelButton: true,
      }).then((confirmed: boolean) => {
        if (confirmed) {
          saveCurrentGroup().then(() => {
            switchToGroup(index);
          });
        } else {
          switchToGroup(index);
        }
      });
    } else {
      // 降级到原生确认对话框
      const confirmed = confirm("是否保存当前分组的更改？");
      if (confirmed) {
        saveCurrentGroup().then(() => {
          switchToGroup(index);
        });
      } else {
        switchToGroup(index);
      }
    }
  } else {
    switchToGroup(index);
  }
};

// 切换到指定分组
const switchToGroup = (index: number) => {
  currentEditingIndex.value = index;
  editorContentChanged.value = false;
  if (editor.value) {
    // 设置编辑器是否可编辑
    editor.value.dispatch({
      effects: editableCompartment.reconfigure(
        EditorView.editable.of(index !== 0)
      ),
    });
    // 设置编辑器内容
    let content = "";
    if (index === 0) {
      content = systemHosts.value;
    } else {
      const group = groups.value[index - 1];
      content = group?.hosts || "";
      // 记录原始内容用于变更检测和撤销
      if (group) {
        group.originalContent = content;
      }
    }
    editor.value.dispatch({
      changes: {
        from: 0,
        to: editor.value.state.doc.length,
        insert: content,
      },
    });
  }
};

// 删除分组
const deleteItem = async (index: number) => {
  const group = groups.value[index];
  const dialogApi = internalInstance?.appContext.config.globalProperties.$dialog;
  let response = false;
  
  if (dialogApi) {
    response = await dialogApi({
      title: "删除确认",
      message: `确定要删除分组 "${group.label}" 吗？此操作不可撤销。`,
      cancelText: "取消",
      confirmText: "删除",
      showCancelButton: true,
    });
  } else {
    response = confirm(`确定要删除分组 "${group.label}" 吗？此操作不可撤销。`);
  }
  
  if (!response) return;
  
  try {
    if (group.active) {
      // 情况1：分组已激活 - 需要从 hosts 文件中删除分组内容
      await invoke("remove_hosts_group", { groupName: group.label });
      showMessage(`已从 hosts 文件中删除分组 "${group.label}"`, "success");
      
      // 刷新系统 hosts 内容
      const sysHosts = await invoke("get_default_hosts") as string;
      systemHosts.value = sysHosts;
      
      // 如果当前正在查看系统 hosts，更新编辑器内容
      if (currentEditingIndex.value === 0 && editor.value) {
        editor.value.dispatch({
          changes: {
            from: 0,
            to: editor.value.state.doc.length,
            insert: sysHosts,
          },
        });
      }
    } else {
      // 情况2：分组未激活 - 只需要从本地删除，不需要操作 hosts 文件
      showMessage(`已删除本地分组 "${group.label}"`, "success");
    }
    
    // 从本地列表和存储中移除分组
    groups.value.splice(index, 1);
    localStorage.setItem("hosts-groups", JSON.stringify(groups.value));
    
    // 如果正在编辑被删除的分组，切换到系统 hosts
    if (currentEditingIndex.value === index + 1) {
      showHostsDetail(0);
    } else if (currentEditingIndex.value > index + 1) {
      currentEditingIndex.value--;
    }
    
  } catch (error: any) {
    console.error("删除分组失败:", error);
    const errorMessage = error?.toString() || "删除分组失败";
    
    if (errorMessage.includes("Insufficient privileges") || errorMessage.includes("Administrator access required")) {
      showMessage("需要管理员权限才能修改 hosts 文件。请以管理员身份重新启动应用程序。", "error");
    } else if (errorMessage.includes("User cancelled elevation")) {
      showMessage("用户取消了权限提升请求", "info");
    } else if (errorMessage.includes("Group not found")) {
      // 如果分组在 hosts 文件中未找到，但本地有记录，仍然删除本地记录
      showMessage(`分组 "${group.label}" 在 hosts 文件中未找到，已从本地删除`, "warning");
      groups.value.splice(index, 1);
      localStorage.setItem("hosts-groups", JSON.stringify(groups.value));
      
      if (currentEditingIndex.value === index + 1) {
        showHostsDetail(0);
      } else if (currentEditingIndex.value > index + 1) {
        currentEditingIndex.value--;
      }
    } else {
      showMessage("删除分组失败：" + errorMessage, "error");
    }
  }
};

// 切换分组激活状态
const toggleGroupActive = async (group: Group, forceDeactivate = false) => {
  try {
    const shouldActivate = forceDeactivate ? false : !group.active;
    if (shouldActivate) {
      // 激活分组
      const content = group.hosts;
      await invoke("add_hosts_fragment_with_group", { 
        groupName: group.label, 
        fragment: content 
      });
      group.active = true;
      showMessage(`已激活分组 "${group.label}"`, "success");
    } else {
      // 停用分组
      await invoke("remove_hosts_group", { groupName: group.label });
      group.active = false;
      showMessage(`已停用分组 "${group.label}"`, "success");
    }
    // 刷新系统 hosts 内容
    const sysHosts = await invoke("get_default_hosts") as string;
    systemHosts.value = sysHosts;
    if (currentEditingIndex.value === 0 && editor.value) {
      editor.value.dispatch({
        changes: {
          from: 0,
          to: editor.value.state.doc.length,
          insert: sysHosts,
        },
      });
    }
  } catch (error: any) {
    console.error("切换分组状态失败:", error);
    const errorMessage = error?.toString() || "操作失败";
    
    if (errorMessage.includes("Insufficient privileges") || errorMessage.includes("Administrator access required")) {
      showMessage("需要管理员权限才能修改 hosts 文件。请以管理员身份重新启动应用程序。", "error");
    } else if (errorMessage.includes("User cancelled elevation")) {
      showMessage("用户取消了权限提升请求", "info");
    } else {
      showMessage("操作失败：" + errorMessage, "error");
    }
  }
};

// 创建新分组
const createNewGroup = async () => {
  const name = newGroupName.value.trim();
  if (!name) {
    showMessage("请输入分组名称", "warning");
    return;
  }
  // 检查是否已存在同名分组
  if (groups.value.some(g => g.label === name)) {
    showMessage("分组名称已存在", "warning");
    return;
  }
  try {
    // 创建空分组
    await invoke("add_hosts_fragment_with_group", { 
      groupName: name, 
      fragment: "# 这是一个新的 hosts 分组\n127.0.0.1    localhost" 
    });
    // 刷新分组列表
    await refreshGroups();
    // 找到新创建的分组并切换到它
    const newGroupIndex = groups.value.findIndex(g => g.label === name);
    if (newGroupIndex !== -1) {
      showHostsDetail(newGroupIndex + 1);
    }
    // 重置添加状态
    cancelAddGroup();
    showMessage("分组创建成功", "success");
  } catch (error: any) {
    console.error("创建分组失败:", error);
    const errorMessage = error?.toString() || "创建分组失败";
    
    if (errorMessage.includes("Insufficient privileges") || errorMessage.includes("Administrator access required")) {
      showMessage("需要管理员权限才能修改 hosts 文件。请以管理员身份重新启动应用程序。", "error");
    } else if (errorMessage.includes("User cancelled elevation")) {
      showMessage("用户取消了权限提升请求", "info");
    } else {
      showMessage("创建分组失败：" + errorMessage, "error");
    }
  }
};

// 取消添加分组
const cancelAddGroup = () => {
  isAddingNewGroup.value = false;
  newGroupName.value = "";
};

// 输入框失去焦点时的处理（延迟取消，避免点击按钮时立即取消）
const onNewGroupInputBlur = () => {
  setTimeout(() => {
    if (isAddingNewGroup.value && !newGroupName.value.trim()) {
      cancelAddGroup();
    }
  }, 100);
};

// 开始添加新分组
const startAddGroup = () => {
  isAddingNewGroup.value = true;
  nextTick(() => {
    newGroupInput.value?.focus();
  });
};

// 保存当前分组
const saveCurrentGroup = async () => {
  if (currentEditingIndex.value === 0) return; // 系统 hosts 不可编辑
  const group = groups.value[currentEditingIndex.value - 1];
  const newContent = editor?.value?.state.doc.toString() || "";
  if (newContent === group.originalContent) {
    showMessage("没有更改需要保存", "info");
    return;
  }
  try {
    // 如果分组当前是激活状态，先停用它
    const wasActive = group.active;
    if (wasActive) {
      await invoke("remove_hosts_group", { groupName: group.label });
    }
    
    // 更新分组内容（不激活）
    group.hosts = newContent;
    group.originalContent = newContent;
    group.active = false; // 保存后不自动激活
    editorContentChanged.value = false;
    
    // 如果之前是激活状态，重新激活更新后的内容
    if (wasActive) {
      await invoke("add_hosts_fragment_with_group", { 
        groupName: group.label, 
        fragment: newContent 
      });
      group.active = true;
      
      // 刷新系统 hosts 内容
      const sysHosts = await invoke("get_default_hosts") as string;
      systemHosts.value = sysHosts;
      if (currentEditingIndex.value === 0 && editor.value) {
        editor.value.dispatch({
          changes: {
            from: 0,
            to: editor.value.state.doc.length,
            insert: sysHosts,
          },
        });
      }
    }
    
    showMessage("保存成功", "success");
  } catch (error: any) {
    console.error("保存失败:", error);
    const errorMessage = error?.toString() || "保存失败";
    
    if (errorMessage.includes("Insufficient privileges") || errorMessage.includes("Administrator access required")) {
      showMessage("需要管理员权限才能修改 hosts 文件。请以管理员身份重新启动应用程序。", "error");
    } else if (errorMessage.includes("User cancelled elevation")) {
      showMessage("用户取消了权限提升请求", "info");
    } else {
      showMessage("保存失败：" + errorMessage, "error");
    }
  }
};

// 撤销更改
const undoChanges = () => {
  if (currentEditingIndex.value === 0 || !editorContentChanged.value) return;
  const group = groups.value[currentEditingIndex.value - 1];
  if (editor.value && group.originalContent !== undefined) {
    editor.value.dispatch({
      changes: {
        from: 0,
        to: editor.value.state.doc.length,
        insert: group.originalContent,
      },
    });
    editorContentChanged.value = false;
  }
};

// 编辑器内容变更处理
const onDocChange = (v: ViewUpdate) => {
  if (currentEditingIndex.value === 0) return; // 系统 hosts 不可编辑
  const doc = v.state.doc.toString();
  const group = groups.value[currentEditingIndex.value - 1];
  if (group) {
    // 检测内容变更
    editorContentChanged.value = doc !== (group.originalContent || "");
  }
};

// 开始编辑分组名称
const startEditGroupName = (index: number, currentName: string) => {
  editingGroupIndex.value = index;
  editingGroupName.value = currentName;
  nextTick(() => {
    // 自动聚焦并选中文本
    const inputs = document.querySelectorAll('input[data-edit-group="true"]');
    const input = inputs[inputs.length - 1] as HTMLInputElement; // 获取最新的输入框
    if (input) {
      input.focus();
      input.select();
    }
  });
};

// 完成编辑分组名称
const finishEditGroupName = async () => {
  if (editingGroupIndex.value === -1) return;
  
  const newName = editingGroupName.value.trim();
  const oldGroup = groups.value[editingGroupIndex.value];
  
  // 验证新名称
  if (!newName) {
    showMessage("分组名称不能为空", "error");
    cancelEditGroupName();
    return;
  }
  
  if (newName === oldGroup.label) {
    // 名称没有变化，直接取消编辑
    cancelEditGroupName();
    return;
  }
  
  // 检查重名
  const exists = groups.value.some((group, index) => 
    index !== editingGroupIndex.value && group.label === newName
  );
  
  if (exists) {
    showMessage("分组名称已存在", "error");
    return;
  }
  
  try {
    const wasActive = oldGroup.active;
    
    // 如果分组当前是激活状态，先从 hosts 中删除旧分组
    if (wasActive) {
      await invoke("remove_hosts_group", { groupName: oldGroup.label });
    }
    
    // 更新分组名称
    oldGroup.label = newName;
    
    // 如果分组是激活状态，用新名称重新添加到 hosts
    if (wasActive) {
      await invoke("add_hosts_fragment_with_group", { 
        groupName: newName, 
        fragment: oldGroup.hosts 
      });
      
      // 刷新系统 hosts 内容
      const sysHosts = await invoke("get_default_hosts") as string;
      systemHosts.value = sysHosts;
      if (currentEditingIndex.value === 0 && editor.value) {
        editor.value.dispatch({
          changes: {
            from: 0,
            to: editor.value.state.doc.length,
            insert: sysHosts,
          },
        });
      }
    }
    
    // 更新本地存储
    localStorage.setItem("hosts-groups", JSON.stringify(groups.value));
    
    // 如果当前正在编辑这个分组，更新标题
    if (currentEditingIndex.value === editingGroupIndex.value + 1) {
      // 触发标题更新
    }
    
    showMessage(`分组已重命名为：${newName}`, "success");
    
  } catch (error: any) {
    // 回滚名称变更
    oldGroup.label = editingGroupName.value; // 恢复原始名称
    console.error("重命名分组失败:", error);
    const errorMessage = error?.toString() || "重命名失败";
    if (errorMessage.includes("Access is denied") || errorMessage.includes("权限")) {
      showMessage("权限不足，请以管理员身份运行", "error");
    } else {
      showMessage(`重命名失败: ${errorMessage}`, "error");
    }
  } finally {
    cancelEditGroupName();
  }
};

// 取消编辑分组名称
const cancelEditGroupName = () => {
  editingGroupIndex.value = -1;
  editingGroupName.value = "";
};

// 初始化
onMounted(async () => {
  try {
    // 获取系统 hosts
    const sysHosts = await invoke("get_default_hosts") as string;
    systemHosts.value = sysHosts;
    
    // 加载并合并所有分组（hosts 文件 + 本地存储）
    await refreshGroups();
    
    // 初始化编辑器
    editor.value = new EditorView({
      extensions: [
        basicSetup,
        StreamLanguage.define(coffeeScript),
        EditorView.updateListener.of(debounce(onDocChange, 300)),
        editableCompartment.of(EditorView.editable.of(false)),
      ],
      doc: sysHosts,
      parent: editorArea.value?.querySelector('.editor-container') as HTMLElement,
    });
    
    // 设置编辑器样式
    nextTick(() => {
      const editorEl = document.querySelector('.cm-editor');
      if (editorEl) {
        editorEl.classList.add('editor-content');
      }
    });
  } catch (error) {
    console.error("初始化失败:", error);
    showMessage("应用初始化失败", "error");
    
    // 即使初始化失败，也尝试加载本地存储的分组
    const localGroupsStr = localStorage.getItem("hosts-groups");
    if (localGroupsStr) {
      try {
        const localGroups: Group[] = JSON.parse(localGroupsStr);
        groups.value = localGroups;
      } catch (parseError) {
        console.error("加载本地分组失败:", parseError);
      }
    }
  }
});

// 监听分组列表变化，保存到本地存储
watch(groups, (newGroups) => {
  localStorage.setItem("hosts-groups", JSON.stringify(newGroups));
}, { deep: true });
</script>

<style scoped>
#main {
  display: flex;
  height: 100vh;
}

.editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.editor-container {
  flex: 1;
  overflow: hidden;
}

.btn-action {
  transition: all 0.2s ease;
}

.btn-action:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

:deep(.cm-editor) {
  height: 100% !important;
}

:deep(.cm-scroller) {
  overflow: auto !important;
}

:deep(.editor-content) {
  height: 100% !important;
}

.group-name {
  font-size: 0.875rem; /* 14px */
  line-height: 1.25rem; /* 20px */
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: inline-block;
  vertical-align: middle;
}
</style>