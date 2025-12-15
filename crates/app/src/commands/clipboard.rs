use ribo_db::models::{NewRecord, NewTarget, Record, RecordQuery, RecordWithTargets, Target};
use tauri::State;

use crate::store::db::Db;

pub type CommandResult<T> = Result<T, String>;

#[tauri::command]
pub fn get_records(
  state: State<'_, Db>,
  query: RecordQuery,
) -> CommandResult<Vec<RecordWithTargets>> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  match db.query_record(query) {
    Ok(records) => Ok(records),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn get_record(state: State<'_, Db>, id: u64) -> CommandResult<Record> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  match db.get_record_by_id(id) {
    Ok(Some(record)) => Ok(record),
    Ok(None) => Err(ribo_db::Error::NotFound(format!("{id}"))),
    Err(e) => Err(e),
  }
  .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_record(state: State<'_, Db>, id: u64) -> CommandResult<()> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  match db.delete_record(id) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn create_record(state: State<'_, Db>, clipboard: NewRecord) -> CommandResult<Record> {
  let db = state.0.lock().map_err(|e| e.to_string())?;

  match db.create_record(clipboard) {
    Ok(record) => Ok(record),
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
