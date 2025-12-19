export const routes = [
  {
    path: "/tray-pane",
    // name: "trayPane",
    component: () => import("@/views/tray-pane/container.vue"),
    children: [
      {
        path: "",
        name: "tray_pane", // 请勿修改
        component: () => import("@/views/tray-pane/index.vue"),
      },
      {
        path: "edit",
        name: "trayPaneEdit",
        component: () => import("@/views/tray-pane/edit.vue"),
      },
      {
        path: "qrcode",
        name: "trayPaneQrcode",
        component: () => import("@/views/tray-pane/qrcode.vue"),
      }
    ]
  },
  {
    path: "/setting",
    name: "setting",
    component: () => import("@/views/setting/index.vue"),
  },
  {
    path: "/context",
    name: "context_pane", // 请勿修改
    component: () => import("@/views/context-pane.vue"),
  },
];
