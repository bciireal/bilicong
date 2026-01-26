import { createApp } from "vue";
import router from "./router";
import "./style.css";
import App from "./App.vue";

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
