use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;
use crate::utils::constant::{STORE_FILE};

pub struct Store;

impl Store {
  pub fn init<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<()> {
    let _store = app.store(STORE_FILE)?;
      // TODO 通知页面刷新
      Ok(())
  }
}