/// <reference types="vite/client" />
type Any = any;

interface ImportMetaEnv {
  readonly VITE_APP_URI_BASE_WIN: string;
  readonly VITE_APP_URI_BASE_NOT_WIN: string;

  // 启用功能标识
  readonly VITE_APP_ENABLE: "true" | "false";
  // 启用功能快捷键标识
  readonly VITE_APP_ENABLE_HOTKRY: "true" | "false";
  // 启用删除功能标识
  readonly VITE_APP_ENABLE_DELETE: "true" | "false";
  // 启用删除功能快捷键标识
  readonly VITE_APP_ENABLE_DELETE_HOTKEY: "true" | "false";
  // 启用执行功能标识
  readonly VITE_APP_ENABLE_EXEC: "true" | "false";
  // 启用执行功能快捷键标识
  readonly VITE_APP_ENABLE_EXEC_HOTKRY: "true" | "false";
  // 启用二维码功能标识
  readonly VITE_APP_ENABLE_QRCODE: "true" | "false";
  // 启用二维码功能快捷键标识
  readonly VITE_APP_ENABLE_QRCODE_HOTKRY: "true" | "false";
  // 启用编辑功能标识
  readonly VITE_APP_ENABLE_EDIT: "true" | "false";
  // 启用编辑功能快捷键标识
  readonly VITE_APP_ENABLE_EDIT_HOTKRY: "true" | "false";
  // 启用复制功能标识
  readonly VITE_APP_ENABLE_COPY: "true" | "false";
  // 启用复制功能快捷键标识
  readonly VITE_APP_ENABLE_COPY_HOTKRY: "true" | "false";
  // 启用上一个功能标识
  readonly VITE_APP_ENABLE_PREV: "true" | "false";
  // 启用上一个功能快捷键标识
  readonly VITE_APP_ENABLE_PREV_HOTKEY: "true" | "false";
  // 启用下一个功能标识
  readonly VITE_APP_ENABLE_NEXT: "true" | "false";
  // 启用下一个功能快捷键标识
  readonly VITE_APP_ENABLE_NEXT_HOTKEY: "true" | "false";
}