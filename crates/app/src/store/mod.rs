use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::StoreExt;
use crate::utils::constant::{APP_NAME, STORE_DB_FILE, STORE_FILE};

pub mod db;

pub struct Store;

impl Store {
  pub fn init<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<()> {
    let _store = app.store(STORE_FILE)?;

    let p = crate::utils::path::get_ribo_db_path(app)?.join(STORE_DB_FILE);

    app.manage(self::db::Db::new(p, Some(APP_NAME.to_string()))?);
      // TODO 通知页面刷新
    Ok(())
  }
}