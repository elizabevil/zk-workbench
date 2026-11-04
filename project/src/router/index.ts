import { createRouter, createWebHashHistory } from "vue-router";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", name: "workbench", component: () => import("../pages/workbench/index.vue") },
  ],
});
