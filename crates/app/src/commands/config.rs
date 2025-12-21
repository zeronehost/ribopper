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
  let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
  if config.get_autostart() {
    app.autolaunch().enable().map_err(|e| e.to_string())?;
  } else {
    app.autolaunch().disable().map_err(|e| e.to_string())?;
  }
  store.set("config", json!(config));
  log::info!("通知前端更新状态");
  RiboEvent::<()>::create_refresh_event(None, WIN_LABEL_TRAY_PANE)
    .emit(&app)
    .map_err(|e| e.to_string())?;
  // if let Some(c) = config.general
  //   && let Some(_max) = c.max
  // {
  //   let _db = state.0.lock().map_err(|e| e.to_string())?;
  //   let len = db.clear_overflow_data(max).map_err(|e| e.to_string())?;
  //   if len > 0 {
  //     RiboEvent::<()>::create_update_event(None, WIN_LABEL_TRAY_PANE)
  //       .emit(&app)
  //       .map_err(|e| e.to_string())?;
  //   }
  // }
  Ok(())
}
