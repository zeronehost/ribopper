use std::path::PathBuf;

use ribo_db::models::{NewRecord, RecordQuery};
use tauri::{AppHandle, Manager, Runtime, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

#[cfg(feature = "action")]
use crate::menu::Context;
use crate::{
  events::{EventAction, EventLabel},
  models::{Record, UpdateRecord},
  store::db::Db,
  utils::{error::Result, path::get_images_path, qrcode::create_qrcode},
};

#[tauri::command]
pub fn get_records(state: State<'_, Db>, query: RecordQuery) -> Result<Vec<Record>> {
  log::debug!(
    "commands::record::get_records called with query={:?}",
    query
  );
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::record::get_records - failed to lock db: {}", e);
    e
  })?;
  let data = db.query_record(query)?;
  data
    .iter()
    .map(|i| {
      i.try_into().map_err(|e: serde_json::Error| {
        log::error!("commands::record::get_records - failed to convert: {}", e);
        e.into()
      })
    })
    .collect()
}

#[tauri::command]
pub fn get_record(state: State<'_, Db>, id: u64) -> Result<Record> {
  log::debug!("commands::record::get_record id={}", id);
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::record::get_record - failed to lock db: {}", e);
    e
  })?;

  let data = db.get_record_by_id(id).map_err(|e| {
    log::error!("commands::record::get_record - db error: {}", e);
    e
  })?;

  let data: Record = match data {
    Some(data) => data.try_into().map_err(|e: serde_json::Error| {
      log::error!("commands::record::get_record - failed to convert: {}", e);
      e
    })?,
    None => {
      log::error!("commands::record::get_record - record not found id={id}");
      return Err(ribo_db::Error::NotFound(format!("{id}")).into());
    }
  };

  Ok(data)
}

#[tauri::command]
pub fn delete_record<R: Runtime>(app: AppHandle<R>, id: u64) -> Result<bool> {
  log::info!("commands::record::delete_record id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::record::delete_record - failed to lock db: {}", e);
    e
  })?;
  let record = db.get_record_by_id(id).map_err(|e| {
    log::error!(
      "commands::record::delete_record - get_record_by_id error: {}",
      e
    );
    e
  })?;

  if let Some(record) = record {
    let path = get_images_path(&app)
      .map_err(|e| {
        log::error!(
          "commands::record::delete_record - get_images_path error: {}",
          e
        );
        e
      })?
      .join(&record.content);
    log::info!(
      "commands::record::delete_record - deleting dir: {:?}",
      path.display()
    );
    if path.exists() {
      let _ = std::fs::remove_file(path).map_err(|e| {
        log::error!(
          "commands::record::delete_record - failed to remove dir: {}",
          e
        );
        e
      });
    }
  }
  match db.delete_record(id) {
    Ok(success) => {
      if success {
        crate::events::RiboEvent::create_update_event(EventLabel::Record, EventAction::Delete)
          .emit(&app)?;
        log::info!("commands::record::delete_record - record deleted id={}", id);
      }
      Ok(success)
    }
    Err(e) => {
      log::error!("commands::record::delete_record - db error: {}", e);
      Err(e.into())
    }
  }
}

#[tauri::command]
pub fn create_record<R: Runtime>(app: AppHandle<R>, clipboard: NewRecord) -> Result<Record> {
  log::info!("commands::record::create_record type={:?}", clipboard.typ);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::record::create_record - failed to lock db: {}", e);
    e
  })?;
  let max = if let Ok(Some(config)) = super::config::config_load(app.clone()) {
    config.get_max().unwrap_or(None)
  } else {
    None
  };

  let data = db.create_record(clipboard, max).map_err(|e| {
    log::error!("commands::record::create_record - db error: {}", e);
    e
  })?;
  let created: Record = data.try_into().map_err(|e: serde_json::Error| {
    log::error!("commands::record::create_record - failed to convert: {}", e);
    e
  })?;
  crate::events::RiboEvent::create_update_event(EventLabel::Record, EventAction::Create)
    .emit(&app)?;
  log::info!("commands::record::create_record - created record");
  Ok(created)
}

#[tauri::command]
pub fn update_record<R: Runtime>(app: AppHandle<R>, record: UpdateRecord) -> Result<bool> {
  log::info!("commands::record::update_record called");
  let state = app.state::<crate::store::db::Db>();
  #[allow(unused_mut)]
  let mut db = state.0.lock().map_err(|e| {
    log::error!("commands::record::update_record - failed to lock db: {}", e);
    e
  })?;
  match record.try_into() {
    Ok((id, content)) => match db.update_record_content(id, content.clone()) {
      Ok(success) => {
        if success {
          #[cfg(feature = "action")]
          db.update_action_option_by_record(id, &content)?;
          crate::events::RiboEvent::create_update_event(EventLabel::Record, EventAction::Update)
            .emit(&app)?;
          log::info!("commands::record::update_record - updated id={}", id);
        }
        Ok(success)
      }
      Err(e) => Err(e.into()),
    },
    Err(e) => Err(e.into()),
  }
}

#[tauri::command]
pub fn clear_records<R: Runtime>(app: AppHandle<R>) -> Result<()> {
  log::info!("commands::record::clear_records called");
  let app_handle = app.clone();
  app
    .dialog()
    .message("确认要清空历史记录？")
    .title("温馨提示")
    .buttons(MessageDialogButtons::OkCancelCustom(
      "确定".to_string(),
      "取消".to_string(),
    ))
    .show(move |result| {
      if result {
        log::info!("commands::record::clear_records confirmed by user");
        // 通知刷新
        let state = app_handle.state::<crate::store::db::Db>();
        let db = state
          .0
          .lock()
          .map_err(|e| {
            log::error!("commands::record::clear_records - failed to lock db: {}", e);
            e
          })
          .unwrap();
        match db.clear_records() {
          Ok(_) => {
            match get_images_path(&app) {
              Ok(path) => {
                if path.exists() {
                  let _ = std::fs::remove_dir_all(path);
                }
              }
              Err(e) => {
                log::error!(
                  "commands::record::clear_records - failed to get images path: {}",
                  e
                );
              }
            }
            match crate::events::RiboEvent::create_update_event(
              EventLabel::Record,
              EventAction::Clear,
            )
            .emit(&app_handle)
            {
              Ok(_) => {
                log::info!("commands::record::clear_records - notify refresh succeeded");
              }
              Err(e) => {
                log::error!(
                  "commands::record::clear_records - notify refresh failed: {}",
                  e
                );
              }
            };
          }
          Err(e) => {
            log::error!("commands::record::clear_records - clear failed: {}", e);
          }
        };
      }
    });
  Ok(())
}

#[tauri::command]
pub fn copy_record<R: Runtime>(app: AppHandle<R>, id: u64) -> Result<()> {
  log::info!("commands::record::copy_record called id={id}");
  let db = app.state::<crate::store::db::Db>();
  let clipboard = app.state::<crate::store::clipboard::Clipboard>();
  let db = db.0.lock().map_err(|e| {
    log::error!("commands::record::copy_record - failed to lock db: {}", e);
    e
  })?;
  let data = db.get_record_by_id(id).map_err(|e| {
    log::error!(
      "commands::record::copy_record - failed to get record: {}",
      e
    );
    e
  })?;
  if let Some(record) = data {
    match record.typ {
      ribo_db::models::RecordType::Text => {
        let content = ribo_clipboard::FormatContent::Text(record.content.clone());
        clipboard.0.paste(ribo_clipboard::Content {
          data: vec![content.clone()],
          content,
        })
      }
      #[cfg(feature = "image")]
      ribo_db::models::RecordType::Image => {
        let p = get_images_path(&app)?.join(record.content);
        if p.exists() {
          let data = image::open(p).map_err(|e| {
            log::error!("commands::record::copy_record - failed to parse image json: {e}");
            e
          })?;
          clipboard.0.paste(ribo_clipboard::Content {
            content: ribo_clipboard::FormatContent::Image(ribo_clipboard::Image {
              width: data.width(),
              height: data.height(),
              data: data.into_rgba8().into_raw(),
            }),
            data: vec![],
          })
        } else {
          log::error!(
            "commands::record::copy_record - image file not exists: {}",
            p.display()
          );
          Ok(())
        }
      }
      #[cfg(feature = "file")]
      ribo_db::models::RecordType::Files => {
        let data = serde_json::from_str::<Vec<ribo_clipboard::FormatContent>>(&record.data)
          .map_err(|e| {
            log::error!("commands::record::copy_record - failed to parse files json: {e}");
            e
          })?;
        clipboard.0.paste(ribo_clipboard::Content {
          content: ribo_clipboard::FormatContent::Files(
            serde_json::from_str::<Vec<PathBuf>>(&record.content).map_err(|e| {
              log::error!("commands::record::copy_record - failed to parse files: {e}");
              e
            })?,
          ),
          data,
        })
      }
    }?;
  }
  Ok(())
}

#[tauri::command]
pub fn qrcode_record<R: Runtime>(app: AppHandle<R>, id: u64) -> Result<Vec<u8>> {
  log::info!("commands::record::qrcode_record called id={id}");
  let state = app.state::<crate::store::db::Db>();

  let record = get_record(state, id)?;
  create_qrcode(
    record,
    #[cfg(feature = "image")]
    &app,
  )
  .map_err(|e| {
    log::error!("commands::record::qrcode_record - failed to create qrcode: {e}");
    e
  })
}
