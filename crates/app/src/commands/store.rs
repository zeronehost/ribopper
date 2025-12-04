use serde_json::json;
use tauri::{AppHandle, Emitter, Runtime};
use tauri_plugin_store::StoreExt;

use crate::utils::constant::STORE_FILE;

#[tauri::command]
pub fn store_set_theme<R: Runtime>(app: AppHandle<R>, theme: &str) -> Result<(), String> {
  let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
  store.set("theme", theme);
  #[cfg(debug_assertions)]
  store.save().map_err(|e| e.to_string())?;
  app
    .emit(
      "ribo-store",
      json!({"event": "update", "data": { "key": "theme", "value": theme }}),
    )
    .map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
pub fn store_load<R: Runtime>(app: AppHandle<R>) -> Result<serde_json::Value, String> {
  let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
  let theme = store.get("theme").or(json!("auto").into());
  Ok(json!({ "theme": theme }))
}