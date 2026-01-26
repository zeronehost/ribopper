/// 应用名
pub const APP_NAME: &str = "ribopper";
/// 应用标题
pub const APP_TITLE: &str = "剪贴板管理工具";
/// 窗口名
pub const WIN_NANE: &str = "剪贴板管理工具-设置";

/// 设置窗口标签
pub const WIN_LABEL_MAIN: &str = "setting";
/// 托盘窗口标签
pub const WIN_LABEL_TRAY_PANE: &str = "tray_pane";
/// 上下文窗口标签
#[allow(unused)]
pub const WIN_LABEL_CONTEXT_PANE: &str = "context_pane";

/// 设置窗口URL
pub const WIN_URL_SETTING: &str = "#/setting";
/// 托盘窗口URL
pub const WIN_URL_TRAY_PANE: &str = "#/tray/";
/// 上下文窗口URL
#[allow(unused)]
pub const WIN_URL_CONTEXT_PANE: &str = "#/context/";

/// 配置文件名
pub const STORE_FILE: &str = if cfg!(debug_assertions) {
  "config.dev.json"
} else {
  "config.json"
};
/// 数据库文件名
pub const STORE_DB_FILE: &str = "ribopper.db";

/// rust通知前端事件
pub const RIBO_EVENT: &str = "ribo-notify";
