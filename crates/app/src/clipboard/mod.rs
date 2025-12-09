use std::{
  sync::{Arc, Mutex},
  thread,
  time::Duration,
};

use crate::store::db::Db;
use ribo_db::models::{HistoryType, NewHistory};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_clipboard_manager::ClipboardExt;

pub struct Clipboard<R: Runtime> {
  app: AppHandle<R>,
  handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
}

impl<R: Runtime> Clipboard<R> {
  pub fn new(app: &AppHandle<R>) -> Self {
    Self {
      app: app.clone(),
      handle: Arc::new(Mutex::new(None)),
    }
  }

  pub fn init(&self) -> anyhow::Result<()> {
    let mut handle = self.handle.lock().unwrap();
    let app_handle = self.app.clone();
    log::info!("init clipboard thread creating...");
    *handle = Some(tauri::async_runtime::spawn(async move {
      let app = app_handle.clone();
      let db = app.try_state::<Db>();
      let clipboard = app_handle.clipboard();
      log::info!("clipboard thread start");
      loop {
        thread::sleep(Duration::from_millis(500));
        if let Ok(content) = clipboard.read_text() {
          log::info!("read clipboard content: {}", content);
          match db {
            Some(ref db) => {
              let database = db.clone();
              let database = database.0.lock().unwrap();
              let res = database.query_datas_by_content(&content).unwrap();
              if res.total == 0 {
                log::info!("database not exist, create new data: {}", content);
                let database1 = db.clone();
                let app_handle = app_handle.clone();
                crate::commands::db::create_data(
                  app_handle,
                  database1,
                  NewHistory {
                    content,
                    typ: HistoryType::Text,
                  },
                )
                .unwrap();
              }
            }
            None => {}
          }
        }
      }
    }));
    Ok(())
  }
}
