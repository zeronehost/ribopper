use crate::{
  store::config::RiboConfig,
  utils::{constant::{APP_NAME, STORE_DB_FILE, STORE_FILE}, error::Result},
};
use serde_json::json;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::StoreExt;

pub(crate) mod clipboard;
pub mod config;
pub mod db;

pub struct Store;

impl Store {
  pub fn init<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    log::info!("store: initializing (file={})", STORE_FILE);
    let store = app.store(STORE_FILE)?;
    if !store.has("config") {
      store.set("config", json!(RiboConfig::default()));
    }

    let p = crate::utils::path::get_ribo_db_path(app)?.join(STORE_DB_FILE);

    log::debug!("store: db path={:?}", p);
    app.manage(self::db::Db::new(p, Some(APP_NAME))?);
    let clipboard = self::clipboard::Clipboard::new(app)?;
    app.manage(clipboard);
    // TODO 通知页面刷新
    Ok(())
  }
}
