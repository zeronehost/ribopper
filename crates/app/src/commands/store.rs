use serde_json::json;
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use crate::{store::config::RiboConfig, utils::constant::STORE_FILE};

#[tauri::command]
pub fn store_load<R: Runtime>(app: AppHandle<R>) -> Result<Option<RiboConfig>, String> {
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
pub fn store_save<R: Runtime>(app: AppHandle<R>, config: RiboConfig) -> Result<(), String> {
  let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
  store.set("config", json!(config));
  Ok(())
}
