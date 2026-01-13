use tauri::{AppHandle, Manager, Runtime};

use crate::{commands::config::config_load, events::EventLabel, utils::error::Result};
pub struct Clipboard(
  pub(crate) ribo_clipboard::Manager<Box<dyn Fn(ribo_clipboard::Content) + Send + 'static>>,
);

impl Clipboard {
  pub fn new<R: Runtime>(app: &AppHandle<R>) -> Result<Self> {
    let app_handle = app.clone();
    let callback: Box<dyn Fn(ribo_clipboard::Content) + Send + 'static> =
      Box::new(move |c: ribo_clipboard::Content| {
        let db = app_handle.state::<crate::store::db::Db>();
        let max = if let Ok(Some(config)) = config_load(app_handle.clone()) {
          config.get_max().unwrap_or(None)
        } else {
          None
        };
        #[allow(unused_mut)]
        let mut db = match db.0.lock() {
          Ok(db) => db,
          Err(e) => {
            log::error!(
              "store::clipboard::Clipboard::new::callback: failed to lock db: {}",
              e
            );
            return;
          }
        };
        let record: ribo_clipboard::Record = match c.try_into() {
          Ok(record) => record,
          Err(e) => {
            log::error!(
              "store::clipboard::Clipboard::new::callback: failed to convert content: {}",
              e
            );
            return;
          }
        };
        match db.create_record(
          ribo_db::models::NewRecord {
            content: record.content,
            data: record.data,
            typ: match record.typ {
              ribo_clipboard::RecordType::Text => ribo_db::models::RecordType::Text,
              #[cfg(feature = "image")]
              ribo_clipboard::RecordType::Image => ribo_db::models::RecordType::Image,
              ribo_clipboard::RecordType::Files => ribo_db::models::RecordType::Files,
            },
          },
          max,
        ) {
          #[allow(unused_variables)]
          Ok(record) => {
            #[cfg(feature = "action")]
            match db.create_action_option_by_record(record.id, &record.content) {
              Ok(_) => {}
              Err(e) => {
                log::error!(
                  "store::clipboard::Clipboard::new::callback: failed to create action option: {}",
                  e
                );
              }
            };
            crate::events::RiboEvent::<()>::create_update_event(None, EventLabel::Record)
              .emit(&app_handle)
              .unwrap();
          }
          Err(e) => log::error!(
            "store::clipboard::Clipboard::new::callback: failed to create record: {}",
            e
          ),
        }
      });

    Ok(Self(ribo_clipboard::Manager::new(callback)?))
  }
}
