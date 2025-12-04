export const routes = [
  {
    path: "/",
    name: "tray_pane",
    component: () => import("@/views/index.vue"),
  },
  {
    path: "/setting",
    name: "setting",
    component: () => import("@/views/setting/container.vue"),
    children: [
      {
        path: "index",
        name: "general",
        component: () => import("@/views/setting/index.vue"),
      },
      {
        path: "options",
        name: "options",
        component: () => import("@/views/setting/options.vue"),
      },
      {
        path: "theme",
        name: "theme",
        component: () => import("@/views/setting/theme.vue"),
      },
      {
        path: "hotkey",
        name: "hotkey",
        component: () => import("@/views/setting/hot-key.vue"),
      },
      {
        path: "helper",
        name: "helper",
        component: () => import("@/views/setting/helper.vue"),
      }
    ]
  },
  {
    path: "/context",
    name: "context_pane",
    component: () => import("@/views/context-pane.vue"),
  },
];
