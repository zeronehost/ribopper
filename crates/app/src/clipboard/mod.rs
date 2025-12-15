// use crate::{
//   events::RiboEvent,
//   store::{config::RiboConfig, db::Db},
//   utils::constant::{APP_NAME, STORE_DB_FILE, STORE_FILE, WIN_LABEL_TRAY_PANE},
// };
// use ribo_db::models::{HistoryType, NewHistory};
// use tauri::{AppHandle, Emitter, Runtime};
// use tauri_plugin_clipboard_manager::ClipboardExt;
// use tauri_plugin_store::StoreExt;

// pub struct Clipboard<R: Runtime> {
//   app: AppHandle<R>,
// }

// impl<R: Runtime> Clipboard<R> {
//   pub fn new(app: AppHandle<R>) -> Self {
//     Self { app }
//   }

//   pub async fn init(&self) -> anyhow::Result<()> {
//     let config = self.app.store(STORE_FILE)?.get("config").unwrap();
//     let config: RiboConfig = serde_json::from_value(config)?;
//     let (max, duration) = match config.general {
//       Some(g) => (g.max, g.duration),
//       None => (None, 500),
//     };
//     let clipboard = self.app.clipboard();
//     let p = crate::utils::path::get_ribo_db_path(&self.app)?.join(STORE_DB_FILE);
//     let db = Db::new(p, Some(APP_NAME.to_string()))?;
//     log::info!("clipboard thread start");
//     loop {
//       tokio::time::sleep(tokio::time::Duration::from_millis(duration)).await;
//       log::debug!("clipboard thread duration {duration}");
//       if let Ok(content) = clipboard.read_text() {
//         log::debug!("read clipboard content: {}", content);
//         let db = db.0.lock().unwrap();
//         let res = db.query_datas_by_content(&content)?;
//         log::debug!("data total: {}", res.total);
//         if res.total == 0 {
//           log::debug!("data not exist, create new data: {}", content);
//           let data = NewHistory {
//             content,
//             typ: HistoryType::Text,
//           };
//           db.create_data(data, max)?;
//           RiboEvent::<()>::create_update_event(None, WIN_LABEL_TRAY_PANE).emit(&self.app)?;
//         }
//       }
//     }
//   }
// }

// pub async fn init(app: AppHandle) -> anyhow::Result<()> {
//   log::info!("init clipboard thread creating...");
//   Clipboard::new(app).init().await?;
//   Ok(())
// }

use tauri::{AppHandle, Manager, Runtime};
pub struct Clipboard<R: Runtime> {
  app: AppHandle<R>,
}

impl<R: Runtime> Clipboard<R> {
  pub fn new(app: AppHandle<R>) -> Self {
    Self { app }
  }

  pub fn init(&self) -> anyhow::Result<()> {
    let app_handle = self.app.clone();
    let cm = ribo_clipboard::Manager::new(move |c: ribo_clipboard::Content| {
      log::info!("clipboard content: {:?}", c);
      let db = app_handle.state::<crate::store::db::Db>();
      db.0
        .lock()
        .unwrap()
        .create_record(c.try_into().unwrap())
        .unwrap();
    })?;
    self.app.manage(cm);
    Ok(())
  }
}
