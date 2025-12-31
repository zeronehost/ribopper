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
    children: [
      {
        path: "",
        name: "setting_general", // 请勿修改
        component: () => import("@/views/setting/general.vue"),
      },
      {
        path: "theme",
        name: "setting_theme", // 请勿修改
        component: () => import("@/views/setting/theme.vue"),
      },
      {
        path: "hot-key",
        name: "setting_hot_key", // 请勿修改
        component: () => import("@/views/setting/hot-key.vue"),
      },
      {
        path: "options",
        name: "setting_options", // 请勿修改
        component: () => import("@/views/setting/options.vue"),
      },
      {
        path: "helper",
        name: "setting_helper", // 请勿修改
        component: () => import("@/views/setting/helper.vue"),
      },
    ]
  },
  {
    path: "/context",
    name: "context_pane", // 请勿修改
    component: () => import("@/views/context-pane.vue"),
  },
];
