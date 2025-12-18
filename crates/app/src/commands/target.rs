use ribo_db::models::{NewTarget, Target};
use tauri::State;

use crate::store::db::Db;
use super::CommandResult;

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
