use std::{fs::create_dir_all, path::PathBuf};

use tauri::{AppHandle, Manager, Runtime};

use super::error::Result;

#[allow(unused)]
pub fn get_ribo_db_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf> {
  #[cfg(debug_assertions)]
  {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("tmp");
    if !p.exists() {
      create_dir_all(&p)?;
    }
    std::fs::write(p.join(".gitignore"), "*")?;
    return Ok(p);
  }
  #[allow(unused)]
  let p = app.path().app_data_dir()?;
  if !p.exists() {
    create_dir_all(&p)?;
  }
  Ok(p)
}

#[allow(unused)]
pub fn get_images_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf> {
  #[cfg(debug_assertions)]
  {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("tmp")
      .join("images");
    if !p.exists() {
      create_dir_all(&p)?;
    }
    return Ok(p);
  }
  #[allow(unused)]
  let p = app.path().app_data_dir()?.join("images");
  if !p.exists() {
    create_dir_all(&p)?;
  }
  Ok(p)
}
