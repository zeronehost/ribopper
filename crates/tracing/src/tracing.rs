use std::path::PathBuf;

use crate::error::Result;
use tauri::{AppHandle, Manager, Runtime};

#[allow(unused)]
pub trait TracingPluginExt<R: Runtime> {
  fn tracing(&self) -> &Tracing<R>;
}

impl<R: Runtime, T: Manager<R>> TracingPluginExt<R> for T {
  fn tracing(&self) -> &Tracing<R> {
    self.state::<Tracing<R>>().inner()
  }
}

pub struct Tracing<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Tracing<R> {
  pub(crate) fn new(app: AppHandle<R>) -> Self {
    Self(app)
  }
  pub fn dirs(&self) -> Result<PathBuf> {
    let p = self.0.path().app_log_dir()?;
    if !p.exists() {
      std::fs::create_dir_all(&p)?;
    }
    Ok(p)
  }
}
