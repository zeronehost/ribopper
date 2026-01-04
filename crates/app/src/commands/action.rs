use ribo_db::models::{Action, ActionWithOption, NewAction, NewActionWithOption, NewRiboOption, RiboOption, UpdateAction, UpdateRiboOption};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
  commands::CommandResult,
  store::db::Db,
  utils::constant::WIN_LABEL_MAIN,
};

#[tauri::command]
pub fn create_action<R: Runtime>(
  app: AppHandle<R>,
  action: NewAction,
) -> CommandResult<Action> {
  log::debug!("commands::action::create_action called");
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_action - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  let action = db.create_action(action).map_err(|e| e.to_string())?;
  crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_MAIN)
    .emit(&app)
    .map_err(|e| e.to_string())?;
  Ok(action)
}

#[tauri::command]
pub fn create_option(
  state: State<'_, Db>,
  option: NewRiboOption,
) -> CommandResult<RiboOption> {
  log::debug!("commands::action::create_option called");
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_option - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  db.create_option(option).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_action_option<R: Runtime>(
  app: AppHandle<R>,
  action: NewActionWithOption,
) -> CommandResult<ActionWithOption> {
  log::debug!("commands::action::create_action_option called");
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_action_option - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  let action = db.create_action_option(action).map_err(|e| e.to_string())?;
  crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_MAIN)
    .emit(&app)
    .map_err(|e| e.to_string())?;
  Ok(action)
}

#[tauri::command]
pub fn get_actions(state: State<'_, Db>) -> CommandResult<Vec<ActionWithOption>> {
  log::debug!("commands::action::get_actions called");
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::get_actions - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  db.get_actions().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_action_by_id(state: State<'_, Db>, id: u64) -> CommandResult<ActionWithOption> {
  log::debug!("commands::action::get_action_by_id called id={}", id);
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::get_action_by_id - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  match db.get_action_by_id(id) {
    Ok(Some(action)) => Ok(action),
    Ok(None) => Err(ribo_db::Error::NotFound(format!("{id}")).to_string()),
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn delete_action<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<bool> {
  log::debug!("commands::action::delete_action called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::delete_action - failed to local db: {}",
      e
    );
    e.to_string()
  })?;
  match db.delete_action(id) {
    Ok(success) => {
      if success {
        crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_MAIN)
          .emit(&app)
          .map_err(|e| e.to_string())?;
        log::info!("commands::action::delete_action - success to delete action id={id}");
      }
      Ok(success)
    }
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn delete_option<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<bool> {
  log::debug!("commands::action::delete_option called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::delete_action - failed to local db: {}",
      e
    );
    e.to_string()
  })?;
  db.delete_option(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_action<R: Runtime>(
  app: AppHandle<R>,
  action: UpdateAction,
) -> CommandResult<bool> {
  log::info!("commands::action::update_action called id={}", action.id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::update_action - failed to local db: {}",
      e
    );
    e.to_string()
  })?;
  db.update_action(action).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_option<R: Runtime>(
  app: AppHandle<R>,
  option: UpdateRiboOption,
) -> CommandResult<bool> {
  log::info!("commands::action::update_option called id={}", option.id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::update_option - failed to local db: {}",
      e
    );
    e.to_string()
  })?;
  db.update_option(option).map_err(|e| e.to_string())

}

#[tauri::command]
pub fn get_exec_action<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<Vec<ActionWithOption>> {
  log::info!("commands::action::exec_action called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::action::exec_action - failed to local db: {}", e);
    e.to_string()
  })?;

  let data = db.get_record_by_id(id).map_err(|e| e.to_string())?;
  match data {
    Some(record) => {
      log::info!("commands::action::exec_action - success to exec action id={}", id);
      let action = db.get_action_record_by_record(&record).map_err(|e| e.to_string())?;
      Ok(action)
    },
    None => {
      log::error!("commands::action::exec_action - record not found id={}", id);
      Ok(vec![])
    }
  }
}

#[tauri::command]
pub fn exec_action<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<bool> {
  log::info!("commands::action::exec_action called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::action::exec_action - failed to local db: {}", e);
    e.to_string()
  })?;

  let data = db.get_action_by_id(id).map_err(|e| e.to_string())?;
  log::debug!("暂未实现, {:?}", data);
  
  // TODO 暂未实现
  Ok(false)
}