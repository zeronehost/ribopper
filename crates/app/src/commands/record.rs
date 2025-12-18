use std::path::PathBuf;

use ribo_db::models::{NewRecord, RecordQuery};
use tauri::{AppHandle, Manager, Runtime, State};

use super::CommandResult;
use crate::{
  models::{Record, RecordWithTargets, UpdateRecord},
  store::db::Db,
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
    .into_iter()
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
pub fn delete_record(state: State<'_, Db>, id: u64) -> CommandResult<bool> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  db.delete_record(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_record(state: State<'_, Db>, clipboard: NewRecord) -> CommandResult<Record> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  let data = db.create_record(clipboard).map_err(|e| e.to_string())?;
  data
    .try_into()
    .map_err(|e: serde_json::Error| e.to_string())
}

#[tauri::command]
pub fn update_record(state: State<'_, Db>, record: UpdateRecord) -> CommandResult<bool> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  match record.try_into() {
    Ok((id, content)) => db
      .update_record_content(id, content)
      .map_err(|e| e.to_string()),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn clear_records(state: State<'_, Db>) -> CommandResult<()> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  db.clear_records().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn copy_record<R: Runtime>(app: AppHandle<R>, id: u64) -> Result<(), String> {
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
