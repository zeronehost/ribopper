use ribo_db::models::{
  Action, ActionWithOption, NewAction, NewActionWithOption, NewRiboOption, RiboOption,
  UpdateAction, UpdateRiboOption,
};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
  events::{EventLabel, RiboEvent},
  store::db::Db,
  utils::error::Result,
  menu::Context,
};

#[tauri::command]
pub fn create_action<R: Runtime>(app: AppHandle<R>, action: NewAction) -> Result<Action> {
  log::debug!("commands::action::create_action called");
  let state = app.state::<crate::store::db::Db>();
  let mut db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_action - failed to local db: {}",
      e
    );
    e
  })?;

  let action = db.create_action(action).map_err(|e| {
    log::error!("commands::action::create_action - failed to create action");
    e
  })?;
  db.create_action_option_by_action(action.id, &action.pattern)?;
  RiboEvent::<()>::create_update_event(None, EventLabel::Action).emit(&app)?;
  Ok(action)
}

#[tauri::command]
pub fn create_option<R: Runtime>(app: AppHandle<R>, option: NewRiboOption) -> Result<RiboOption> {
  log::debug!("commands::action::create_option called");
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_option - failed to local db: {}",
      e
    );
    e
  })?;

  let res = db.create_option(option).map_err(|e| {
    log::error!("commands::action::create_option - failed to create option");
    e
  })?;

  RiboEvent::<()>::create_update_event(None, EventLabel::Option).emit(&app)?;

  Ok(res)
}

#[tauri::command]
pub fn create_action_option<R: Runtime>(
  app: AppHandle<R>,
  action: NewActionWithOption,
) -> Result<ActionWithOption> {
  log::debug!("commands::action::create_action_option called");
  let state = app.state::<crate::store::db::Db>();
  let mut db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_action_option - failed to local db: {}",
      e
    );
    e
  })?;
  log::debug!("create_action_option - action={:?}", action);
  let action_option = db.create_action_option(action.clone()).map_err(|e| {
    log::error!("commands::action::create_action_option - failed to create action option");
    e
  })?;
  db.create_action_option_by_action(action_option.id, action.pattern.as_str())?;
  RiboEvent::<()>::create_update_event(None, EventLabel::ActionOption).emit(&app)?;
  Ok(action_option)
}

#[tauri::command]
pub fn get_actions(state: State<'_, Db>) -> Result<Vec<ActionWithOption>> {
  log::debug!("commands::action::get_actions called");
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::action::get_actions - failed to local db: {}", e);
    e
  })?;

  let data = db.get_action_options().map_err(|e| {
    log::error!("commands::action::get_action_options - failed to get actions");
    e
  })?;
  Ok(data)
}

#[tauri::command]
pub fn get_action_by_id(state: State<'_, Db>, id: u64) -> Result<ActionWithOption> {
  log::debug!("commands::action::get_action_by_id called id={}", id);
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::get_action_by_id - failed to local db: {}",
      e
    );
    e
  })?;

  match db.get_action_by_id(id) {
    Ok(Some(action)) => Ok(action),
    Ok(None) => {
      log::error!("commands::action::get_action_by_id - failed to get action id={id}");
      Err(ribo_db::Error::NotFound(format!("{id}")).into())
    }
    Err(e) => {
      log::error!("commands::action::get_action_by_id - failed to get action id={id}");
      Err(e.into())
    }
  }
}

#[tauri::command]
pub fn get_options_by_action_id(state: State<'_, Db>, id: u64) -> Result<Vec<RiboOption>> {
  log::debug!(
    "commands::action::get_options_by_action_id called id={}",
    id
  );
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::get_options_by_action_id - failed to local db: {}",
      e
    );
    e
  })?;
  let data = db.get_options_by_action_id(id).map_err(|e| {
    log::error!("commands::action::get_options_by_action_id - failed to get options id={id}");
    e
  })?;
  Ok(data)
}

#[tauri::command]
pub fn delete_action<R: Runtime>(app: AppHandle<R>, id: u64) -> Result<bool> {
  log::debug!("commands::action::delete_action called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::delete_action - failed to local db: {}",
      e
    );
    e
  })?;
  match db.delete_action(id) {
    Ok(success) => {
      if success {
        RiboEvent::<()>::create_update_event(None, EventLabel::ActionOption).emit(&app)?;
        log::info!("commands::action::delete_action - success to delete action id={id}");
      }
      Ok(success)
    }
    Err(e) => {
      log::error!("commands::action::delete_action - failed to delete action id={id}");
      Err(e.into())
    }
  }
}

#[tauri::command]
pub fn delete_option<R: Runtime>(app: AppHandle<R>, id: u64) -> Result<bool> {
  log::debug!("commands::action::delete_option called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::delete_action - failed to local db: {}",
      e
    );
    e
  })?;
  match db.delete_option(id) {
    Ok(success) => {
      if success {
        RiboEvent::<()>::create_update_event(None, EventLabel::Option)
          .emit(&app)?;
        log::info!("commands::action::delete_option - success to delete option id={id}");
      }
      Ok(success)
    }
    Err(e) => {
      log::error!("commands::action::delete_option - failed to delete option id={id}");
      Err(e.into())
    }
  }
}

#[tauri::command]
pub fn update_action<R: Runtime>(app: AppHandle<R>, action: UpdateAction) -> Result<bool> {
  log::info!("commands::action::update_action called id={}", action.id);
  let state = app.state::<crate::store::db::Db>();
  let mut db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::update_action - failed to local db: {}",
      e
    );
    e
  })?;
  let id = action.id;
  let binding = action.clone();
  let pattern = binding.pattern.as_str();
  match db.update_action(action) {
    Ok(success) => {
      if success {
        db.update_action_option_by_action(id, pattern)?;
        RiboEvent::<()>::create_update_event(None, EventLabel::Action)
          .emit(&app)?;
        log::info!(
          "commands::action::update_action - success to update action id={}",
          id
        );
      }
      Ok(success)
    }
    Err(e) => {
      log::error!(
        "commands::action::update_action - failed to update action id={}",
        id
      );
      Err(e.into())
    }
  }
}

#[tauri::command]
pub fn update_option<R: Runtime>(app: AppHandle<R>, option: UpdateRiboOption) -> Result<bool> {
  log::info!("commands::action::update_option called id={}", option.id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::update_option - failed to local db: {}",
      e
    );
    e
  })?;
  let id = option.id;
  match db.update_option(option) {
    Ok(success) => {
      if success {
        RiboEvent::<()>::create_update_event(None, EventLabel::Option)
          .emit(&app)?;
        log::info!(
          "commands::action::update_option - success to update option id={}",
          id
        );
      }
      Ok(success)
    }
    Err(e) => {
      log::error!(
        "commands::action::update_option - failed to update option id={}",
        id
      );
      Err(e.into())
    }
  }
}

#[tauri::command]
pub fn show_record_actions<R: Runtime>(app: AppHandle<R>, id: u64, label: &str) -> Result<()> {
  log::info!("commands::record::show_record_action called id={id}");
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::record::show_record_action - failed to lock db: {}",
      e
    );
    e
  })?;
  let record = db
    .get_record_by_id(id)
    .map_err(|e| {
      log::info!(
        "commands::record::show_record_action - failed to get record: {}",
        e
      );
      e
    })?
    .ok_or_else(|| {
      log::info!("commands::record::show_record_action - record not found");
      ribo_db::Error::NotFound("record".to_string())
    })?;
  #[cfg(feature = "action")]
  let actions = db.get_actions_by_record_id(id).map_err(|e| {
    log::info!(
      "commands::record::show_record_action - failed to get actions: {}",
      e
    );
    e
  })?;

  let app = app.app_handle();
  let mut ctx = Context::new(app, &record.content)?;
  ctx.set_menu(actions)?;
  ctx.show(label)?;
  Ok(())
}
