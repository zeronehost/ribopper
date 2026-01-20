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
        let record: ribo_db::models::NewRecord = match c.content {
          ribo_clipboard::FormatContent::Text(text) => ribo_db::models::NewRecord {
            content: text.clone(),
            data: text,
            typ: ribo_db::models::RecordType::Text,
          },
          ribo_clipboard::FormatContent::Files(files) => {
            let data = match serde_json::to_string(&files) {
              Ok(data) => data,
              Err(e) => {
                log::error!(
                  "store::clipboard::Clipboard::new::callback: failed to serialize files: {}",
                  e
                );
                return;
              }
            };
            ribo_db::models::NewRecord {
              content: data.clone(),
              data,
              typ: ribo_db::models::RecordType::Files,
            }
          }
          #[cfg(feature = "image")]
          ribo_clipboard::FormatContent::Image(img) => {
            use crate::utils::path::get_images_path;

            let id = uuid::Uuid::new_v4();
            let filename = if cfg!(debug_assertions) {
              format!("{id}.png")
            } else {
              id.to_string()
            };
            let p = match get_images_path(&app_handle) {
              Ok(p) => p.join(filename.clone()),
              Err(e) => {
                log::error!(
                  "store::clipboard::Clipboard::new::callback: failed to get images path: {}",
                  e
                );
                return;
              }
            };
            let image = image::ImageBuffer::from_raw(img.width, img.height, img.data);
            if let Some(image) = image {
              match image::DynamicImage::ImageRgba8(image).save(p) {
                Ok(_) => ribo_db::models::NewRecord {
                  content: filename.clone(),
                  data: filename,
                  typ: ribo_db::models::RecordType::Image,
                },
                Err(e) => {
                  log::error!(
                    "store::clipboard::Clipboard::new::callback: failed to save image: {}",
                    e
                  );
                  return;
                }
              }
            } else {
              log::error!(
                "store::clipboard::Clipboard::new::callback: failed to load image",
              );
              return;
            }
          }
        };
        match db.create_record(record, max) {
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
