use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub fn close_window<R: Runtime>(app: AppHandle<R>, label: &str) -> Result<(), String> {
  log::info!("commands::window::close_window called for label={}", label);
  if let Some(window) = app.get_webview_window(label) {
    match window.close() {
      Ok(_) => {}
      Err(e) => {
        log::error!("Failed to close window: {}", e);
        return Err(e.to_string());
      }
    }
  };
  log::info!(
    "commands::window::close_window succeeded for label={}",
    label
  );
  Ok(())
}

#[tauri::command]
pub fn web_log(level: Level, message: String) -> Result<(), String> {
  match level {
    Level::Trace => log::trace!("[web] {}", message),
    Level::Debug => log::debug!("[web] {}", message),
    Level::Info => log::info!("[web] {}", message),
    Level::Warn => log::warn!("[web] {}", message),
    Level::Error => log::error!("[web] {}", message),
  }
  Ok(())
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Level {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}
