import { createRouter, createWebHashHistory } from "vue-router";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", name: "home", component: () => import("@/pages/home/HomePage.vue") },
    {
      path: "/workspace",
      name: "workspace",
      component: () => import("@/pages/workspace/WorkspacePage.vue"),
    },
    {
      path: "/audit",
      name: "audit",
      component: () => import("@/pages/audit/AuditPage.vue"),
    },
    {
      path: "/observability",
      name: "observability",
      component: () => import("@/pages/observability/ObservabilityPage.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/pages/settings/SettingsPage.vue"),
    },
  ],
});
