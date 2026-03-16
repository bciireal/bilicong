import { createApp } from "vue";
import router from "./router";
import "./style.css";
import App from "./App.vue";

import { webviewWindow } from "@tauri-apps/api";
import { getProjectVersion } from "./services/api";

window.addEventListener(
  "contextmenu",
  (e) => {
    e.preventDefault();
    e.stopPropagation();
  },
  true,
);

window.addEventListener(
  "keydown",
  (e) => {
    e.preventDefault();
    e.stopPropagation();
  },
  true,
);

createApp(App).use(router).mount("#app");

webviewWindow
  .getCurrentWebviewWindow()
  .setTitle(`BiliCong ${await getProjectVersion()}`);
