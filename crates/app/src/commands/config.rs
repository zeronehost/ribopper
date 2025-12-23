use serde_json::json;
use tauri::{AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_store::StoreExt;

use crate::{
  events::RiboEvent,
  store::config::RiboConfig,
  utils::constant::{STORE_FILE, WIN_LABEL_TRAY_PANE},
};

#[tauri::command]
pub fn config_load<R: Runtime>(app: AppHandle<R>) -> Result<Option<RiboConfig>, String> {
  let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
  match store.get("config") {
    Some(config) => {
      let conf: RiboConfig = serde_json::from_value(config).map_err(|e| e.to_string())?;

      Ok(Some(conf))
    }
    None => Ok(None),
  }
}

#[tauri::command]
pub fn config_save<R: Runtime>(app: AppHandle<R>, config: RiboConfig) -> Result<(), String> {
  log::info!("保存配置 => {:?}", config);
  let store = app.store(STORE_FILE).map_err(|e| {
    log::error!("保存配置失败 => {:?}", e);
    e.to_string()
  })?;
  if config.get_autostart() {
    let _ = app.autolaunch().enable().map_err(|e| {
      log::error!("保存配置 -> 开启开机自启失败 => {:?}", e);
      e.to_string()
    });
  } else {
    let _ = app.autolaunch().disable().map_err(|e| {
      log::error!("保存配置 -> 关闭开机自启失败 => {:?}", e);
      e.to_string()
    });
  }
  store.set("config", json!(config));
  log::info!("通知前端更新状态");
  RiboEvent::<()>::create_refresh_event(None, WIN_LABEL_TRAY_PANE)
    .emit(&app)
    .map_err(|e| {
      log::error!("保存配置 -> 通知前端更新状态失败 => {e:?}");
      e.to_string()
    })?;
  Ok(())
}
