import { createApp, ComponentPublicInstance } from "vue";
import "./styles.css";
import App from "./App.vue";
import Dialog from "./components/Dialog.vue";

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

app.mount("#app");
