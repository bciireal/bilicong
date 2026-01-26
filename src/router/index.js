import { createRouter, createWebHistory } from "vue-router";

import HomeView from "../views/HomeView.vue";
import ListView from "../views/ListView.vue";
import PullView from "../views/PullView.vue";

const routes = [
  {
    path: "/",
    name: "home",
    component: HomeView,
  },
  {
    path: "/list",
    name: "list",
    component: ListView,
  },
  {
    path: "/pull",
    name: "pull",
    component: PullView,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
