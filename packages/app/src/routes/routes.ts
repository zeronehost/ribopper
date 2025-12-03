export const routes = [
  {
    path: "/",
    name: "tray_pane",
    component: () => import("@/views/index.vue"),
  },
  {
    path: "/setting",
    name: "setting",
    component: () => import("@/views/setting.vue"),
  },
  {
    path: "/context",
    name: "context_pane",
    component: () => import("@/views/context-pane.vue"),
  },
];
