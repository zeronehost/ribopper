use ribo_db::models::{NewRecord, NewTarget, RecordQuery, Target};
use tauri::State;

use crate::{
  models::{Record, RecordWithTargets, UpdateRecord},
  store::db::Db,
};

pub type CommandResult<T> = Result<T, String>;

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
pub fn get_targets(state: State<'_, Db>) -> CommandResult<Vec<Target>> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  match db.get_targets() {
    Ok(targets) => Ok(targets),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn create_target(state: State<'_, Db>, target: NewTarget) -> CommandResult<Target> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  match db.create_target(target) {
    Ok(target) => Ok(target),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn delete_target(state: State<'_, Db>, id: i64) -> CommandResult<()> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  match db.delete_target(id) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}
