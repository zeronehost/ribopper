export const routes = [
  {
    path: "/",
    name: "main",
    component: () => import("@/views/index.vue"),
  },
  {
    path: "/setting",
    name: "setting",
    component: () => import("@/views/setting.vue"),
  },
  {
    path: "/tray",
    name: "tray",
    component: () => import("@/views/tray-pane.vue"),
  },
  {
    path: "/context",
    name: "context",
    component: () => import("@/views/context-pane.vue"),
  },
];
