use ribo_db::models::{NewTarget, Target};
use tauri::State;

use super::CommandResult;
use crate::store::db::Db;

#[tauri::command]
pub fn get_targets(state: State<'_, Db>) -> CommandResult<Vec<Target>> {
  log::debug!("commands::target::get_targets called");
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::target::get_targets - failed to lock db: {}", e);
    e.to_string()
  })?;

  match db.get_targets() {
    Ok(targets) => {
      log::debug!("commands::target::get_targets - returning {} targets", targets.len());
      Ok(targets)
    }
    Err(e) => {
      log::error!("commands::target::get_targets - db error: {}", e);
      Err(e.to_string())
    }
  }
}

#[tauri::command]
pub fn create_target(state: State<'_, Db>, target: NewTarget) -> CommandResult<Target> {
  log::info!("commands::target::create_target name={}", target.name);
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::target::create_target - failed to lock db: {}", e);
    e.to_string()
  })?;

  match db.create_target(target) {
    Ok(target) => {
      log::info!("commands::target::create_target - created");
      Ok(target)
    }
    Err(e) => {
      log::error!("commands::target::create_target - db error: {}", e);
      Err(e.to_string())
    }
  }
}

#[tauri::command]
pub fn delete_target(state: State<'_, Db>, id: i64) -> CommandResult<()> {
  log::info!("commands::target::delete_target id={}", id);
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::target::delete_target - failed to lock db: {}", e);
    e.to_string()
  })?;

  match db.delete_target(id) {
    Ok(_) => {
      log::info!("commands::target::delete_target - deleted id={}", id);
      Ok(())
    }
    Err(e) => {
      log::error!("commands::target::delete_target - db error: {}", e);
      Err(e.to_string())
    }
  }
}
