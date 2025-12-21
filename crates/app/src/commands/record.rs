use std::path::PathBuf;

use ribo_db::models::{NewRecord, RecordQuery};
use tauri::{AppHandle, Manager, Runtime, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

use super::CommandResult;
use crate::{
  models::{Record, RecordWithTargets, UpdateRecord},
  store::db::Db,
  utils::constant::WIN_LABEL_TRAY_PANE,
};

#[tauri::command]
pub fn get_records(
  state: State<'_, Db>,
  query: RecordQuery,
) -> CommandResult<Vec<RecordWithTargets>> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  let data = db.query_record(query).map_err(|e| e.to_string())?;
  data
    .iter()
    .map(|i| i.try_into().map_err(|e: serde_json::Error| e.to_string()))
    .collect()
}

#[tauri::command]
pub fn get_record(state: State<'_, Db>, id: u64) -> CommandResult<Record> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  let data = db.get_record_by_id(id).map_err(|e| e.to_string())?;

  let data: Record = match data {
    Some(data) => data
      .try_into()
      .map_err(|e: serde_json::Error| e.to_string())?,
    None => return Err(ribo_db::Error::NotFound(format!("{id}")).to_string()),
  };

  Ok(data)
}

#[tauri::command]
pub fn delete_record<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<bool> {
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| e.to_string())?;

  match db.delete_record(id) {
    Ok(success) => {
      if success {
        crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_TRAY_PANE)
          .emit(&app)
          .map_err(|e| e.to_string())?;
      }
      Ok(success)
    }
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn create_record<R: Runtime>(app: AppHandle<R>, clipboard: NewRecord) -> CommandResult<Record> {
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| e.to_string())?;
  let max = if let Ok(Some(config)) = super::config::config_load(app.clone()) {
    config.get_max().unwrap_or(None)
  } else {
    None
  };

  let data = db
    .create_record(clipboard, max)
    .map_err(|e| e.to_string())?;
  match data.try_into() {
    Ok(data) => {
      crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_TRAY_PANE)
        .emit(&app)
        .map_err(|e| e.to_string())?;
      Ok(data)
    }
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn update_record<R: Runtime>(app: AppHandle<R>, record: UpdateRecord) -> CommandResult<bool> {
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| e.to_string())?;
  match record.try_into() {
    Ok((id, content)) => match db.update_record_content(id, content) {
      Ok(success) => {
        if success {
          crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_TRAY_PANE)
            .emit(&app)
            .map_err(|e| e.to_string())?;
        }
        Ok(success)
      }
      Err(e) => Err(e.to_string()),
    },
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn clear_records<R: Runtime>(app: AppHandle<R>) -> CommandResult<()> {
  log::info!("清楚历史记录");
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
        log::info!("确认清空历史记录");
        // 通知刷新
        let state = app_handle.state::<crate::store::db::Db>();
        let db = state.0.lock().unwrap();
        match db.clear_records() {
          Ok(_) => {
            match crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_TRAY_PANE)
              .emit(&app_handle)
            {
              Ok(_) => {
                log::info!("通知刷新成功");
              }
              Err(e) => {
                log::error!("通知刷新失败: {e}");
              }
            };
          }
          Err(e) => {
            log::error!("清空历史记录失败: {e}");
          }
        };
      }
    });
  Ok(())
}

#[tauri::command]
pub fn copy_record<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<()> {
  log::info!("复制数据: {id}");
  let db = app.state::<crate::store::db::Db>();
  let clipboard = app.state::<crate::store::clipboard::Clipboard>();
  let db = db.0.lock().map_err(|e| e.to_string())?;
  let data = db.get_record_by_id(id).map_err(|e| e.to_string())?;
  log::debug!("data => {:?}", data);
  if let Some(record) = data {
    match record.typ {
      ribo_db::models::RecordType::Text => {
        let content = ribo_clipboard::FormatContent::Text(record.content.clone());
        clipboard.0.paste(ribo_clipboard::Content {
          data: vec![content.clone()],
          content,
        })
      }
      ribo_db::models::RecordType::Image => {
        let data = serde_json::from_str::<Vec<ribo_clipboard::FormatContent>>(&record.data)
          .map_err(|e| e.to_string())?;
        clipboard.0.paste(ribo_clipboard::Content {
          content: ribo_clipboard::FormatContent::Image(
            serde_json::from_str(&record.content).map_err(|e| {
              log::error!("解析图片失败: {e}");
              e.to_string()
            })?,
          ),
          data,
        })
      }
      ribo_db::models::RecordType::Files => {
        let data = serde_json::from_str::<Vec<ribo_clipboard::FormatContent>>(&record.data)
          .map_err(|e| e.to_string())?;
        clipboard.0.paste(ribo_clipboard::Content {
          content: ribo_clipboard::FormatContent::Files(
            serde_json::from_str::<Vec<PathBuf>>(&record.content).map_err(|e| {
              log::error!("解析文件列表失败: {e}");
              e.to_string()
            })?,
          ),
          data,
        })
      }
    }
    .map_err(|e| e.to_string())?;
  }
  Ok(())
}
