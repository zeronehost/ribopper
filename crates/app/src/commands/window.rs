use crate::utils::error::Result;
use tauri::{AppHandle, Manager, Runtime};
use tracing::instrument;

#[tauri::command]
#[instrument(skip(app))]
pub fn close_window<R: Runtime>(app: AppHandle<R>, label: &str) -> Result<()> {
  if let Some(window) = app.get_webview_window(label) {
    match window.close() {
      Ok(_) => {}
      Err(e) => {
        tracing::error!("Failed to close window: {}", e);
        return Err(e.into());
      }
    }
  };
  tracing::info!(
    "commands::window::close_window succeeded for label={}",
    label
  );
  Ok(())
}

#[tauri::command]
pub fn web_log(level: Level, message: String) -> Result<()> {
  match level {
    Level::Trace => tracing::trace!("[web] {}", message),
    Level::Debug => tracing::debug!("[web] {}", message),
    Level::Info => tracing::info!("[web] {}", message),
    Level::Warn => tracing::warn!("[web] {}", message),
    Level::Error => tracing::error!("[web] {}", message),
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
