use ribo_db::models::{Command, NewCommand, UpdateCommand};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
  commands::CommandResult,
  store::db::Db,
  utils::constant::WIN_LABEL_MAIN,
};

#[tauri::command]
pub fn create_command<R: Runtime>(
  app: AppHandle<R>,
  command: NewCommand,
) -> CommandResult<Command> {
  log::debug!("commands::command::create_command called");
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::command::create_command - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  let command = db.create_command(&command).map_err(|e| e.to_string())?;
  crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_MAIN)
    .emit(&app)
    .map_err(|e| e.to_string())?;
  Ok(command)
}

#[tauri::command]
pub fn get_commands(state: State<'_, Db>) -> CommandResult<Vec<Command>> {
  log::debug!("commands::command::get_commands called");
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::command::get_commands - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  db.get_commands().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_command_by_id(state: State<'_, Db>, id: u64) -> CommandResult<Command> {
  log::debug!("commands::command::get_command_by_id called id={}", id);
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::command::get_command_by_id - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  match db.get_command_by_id(id) {
    Ok(Some(command)) => Ok(command),
    Ok(None) => Err(ribo_db::Error::NotFound(format!("{id}")).to_string()),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn delete_command<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<bool> {
  log::debug!("commands::command::delete_command called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::command::delete_command - failed to local db: {}",
      e
    );
    e.to_string()
  })?;
  match db.delete_command(id) {
    Ok(success) => {
      if success {
        crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_MAIN)
          .emit(&app)
          .map_err(|e| e.to_string())?;
        log::info!("commands::command::delete_command - success to delete command id={id}");
      }
      Ok(success)
    }
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn update_command<R: Runtime>(
  app: AppHandle<R>,
  command: UpdateCommand,
) -> CommandResult<bool> {
  log::info!("commands::command::update_command called id={}", command.id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::command::update_command - failed to local db: {}",
      e
    );
    e.to_string()
  })?;
  Ok(false)
  // TODO 暂未实现
  // match db.update_command(command) {
  //   Ok(success) => {
  //     if success {
  //       crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_MAIN)
  //         .emit(&app)
  //         .map_err(|e| e.to_string())?;
  //       log::info!(
  //         "commands::command::update_command - success to update command id={}",
  //         command.id
  //       );
  //     }
  //     Ok(success)
  //   }
  //   Err(e) => Err(e.to_string()),
  // }
}

#[tauri::command]
pub fn exec_command<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<bool> {
  log::info!("commands::command::exec_command called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::command::exec_command - failed to local db: {}", e);
    e.to_string()
  })?;

  let data = db.get_record_by_id(id).map_err(|e| e.to_string())?;
  log::debug!("暂未实现, {:?}", data);
  // TODO 暂未实现
  Ok(false)
}