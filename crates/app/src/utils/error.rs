#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
  #[error("Tauri error: {0}")]
  TauriError(#[from] tauri::Error),
  #[error("Database error: {0}")]
  DatabaseError(#[from] ribo_db::Error),
  #[error("Clipboard error: {0}")]
  ClipboardError(#[from] ribo_clipboard::Error),
  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),
  #[error("QR code error: {0}")]
  QrCodeError(#[from] qrcode::types::QrError),
  #[error("QR code image error: {0}")]
  QrCodeImageError(#[from] image::error::ImageError),
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error("JSON error: {0}")]
  JsonError(#[from] serde_json::Error),
  #[error("Internal error: {0}")]
  Internal(String),
  #[error("Plugin store error: {0}")]
  TauriPluginStoreError(#[from] tauri_plugin_store::Error),
  #[error("Global shortcut error: {0}")]
  TauriPluginShortcutError(#[from] tauri_plugin_global_shortcut::Error),
  #[error("Updater error: {0}")]
  TauriPluginUpdaterError(#[from] tauri_plugin_updater::Error),
}

impl From<Error> for tauri::ipc::InvokeError {
  fn from(err: Error) -> Self {
    tauri::ipc::InvokeError::from(err.to_string())
  }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
  fn from(err: std::sync::PoisonError<T>) -> Self {
    Error::Internal(format!("Mutex lock failed: {}", err))
  }
}

pub type Result<T> = std::result::Result<T, Error>;
