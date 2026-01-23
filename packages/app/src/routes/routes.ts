export const routes = [
  {
    path: "/tray",
    name: "tray",
    component: () => import("@/views/pane/container.vue"),
    children: [
      {
        path: "",
        name: "trayPane",
        component: () => import("@/views/pane/tray-pane.vue"),
      },
      {
        path: "edit",
        name: "trayPaneEdit",
        component: () => import("@/views/pane/edit.vue"),
      },
      {
        path: "qrcode",
        name: "trayPaneQrcode",
        component: () => import("@/views/pane/qrcode.vue"),
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
        name: "setting_general",
        component: () => import("@/views/setting/general.vue"),
      },
      {
        path: "theme",
        name: "setting_theme",
        component: () => import("@/views/setting/theme.vue"),
      },
      {
        path: "hot-key",
        name: "setting_hot_key",
        component: () => import("@/views/setting/hot-key.vue"),
      },
      {
        path: "options",
        name: "setting_options",
        component: () => import("@/views/setting/options.vue"),
      },
      {
        path: "helper",
        name: "setting_helper",
        component: () => import("@/views/setting/helper.vue"),
      },
    ]
  },
  {
    path: "/context",
    name: "context",
    component: () => import("@/views/pane/container.vue"),
    children: [
      {
        path: "",
        name: "contextPane",
        component: () => import("@/views/pane/context-pane.vue"),
      },
      {
        path: "edit",
        name: "contextPaneEdit",
        component: () => import("@/views/pane/edit.vue"),
      },
      {
        path: "qrcode",
        name: "contextPaneQrcode",
        component: () => import("@/views/pane/qrcode.vue"),
      }
    ]
  },
];
