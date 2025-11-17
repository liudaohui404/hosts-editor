import { createApp, ComponentPublicInstance } from "vue";
import "./styles.css";
import App from "./App.vue";
import Dialog from "./components/Dialog.vue";
import Message from "./components/Message.vue";

const app = createApp(App);

interface DialogInstance extends ComponentPublicInstance {
  open: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

const dialog = (options: object = {}) => {
  return new Promise((resolve) => {
    const app = createApp(Dialog, {
      ...options,
      afterLeave() {
        app?.unmount();
      },
    });

    const instance = app.mount(document.createElement("div")) as DialogInstance;
    instance.open = true;

    document.body.appendChild(instance.$el);

    instance.onConfirm = () => {
      instance.open = false;
      resolve(true);
    };
    instance.onCancel = () => {
      instance.open = false;
      resolve(false);
    };
  });
};

dialog.confirm = dialog;

app.config.globalProperties.$dialog = dialog;

interface MessageInstance extends ComponentPublicInstance {
  show: () => void;
  close: () => void;
}

const message = (options: { message?: string; type?: string; duration?: number } = {}) => {
  const { message: msg = "Message", type = "info", duration = 3000 } = options;
  const messageApp = createApp(Message, {
    message: msg,
    type,
    duration,
  });

  const instance = messageApp.mount(document.createElement("div")) as MessageInstance;
  document.body.appendChild(instance.$el);

  instance.show();

  return instance;
};

message.success = (msg: string, duration?: number) =>
  message({ message: msg, type: "success", duration });
message.error = (msg: string, duration?: number) =>
  message({ message: msg, type: "error", duration });
message.warning = (msg: string, duration?: number) =>
  message({ message: msg, type: "warning", duration });
message.info = (msg: string, duration?: number) =>
  message({ message: msg, type: "info", duration });

app.config.globalProperties.$message = message;

app.mount("#app");
