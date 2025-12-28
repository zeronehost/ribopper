use std::{fs::create_dir_all, path::PathBuf};

use tauri::{AppHandle, Manager, Runtime};

pub fn get_ribo_db_path<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<PathBuf> {
  let p = app.path().app_data_dir()?;
  if !p.exists() {
    create_dir_all(&p)?;
  }
  Ok(p)
}
