use std::path::PathBuf;

use tauri::{AppHandle, Manager, Runtime};

pub fn get_ribo_db_path<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<PathBuf> {
  Ok(app.path().app_data_dir()?)
}
