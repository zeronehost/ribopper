use ribo_db::models::{
  Action, ActionWithOption, NewAction, NewActionWithOption, NewRiboOption, RiboOption,
  UpdateAction, UpdateRiboOption,
};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
  commands::CommandResult,
  events::{EventLabel, RiboEvent},
  store::db::Db,
};

#[tauri::command]
pub fn create_action<R: Runtime>(app: AppHandle<R>, action: NewAction) -> CommandResult<Action> {
  log::debug!("commands::action::create_action called");
  let state = app.state::<crate::store::db::Db>();
  let mut db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_action - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  let action = db.create_action(action).map_err(|e| {
    log::error!("commands::action::create_action - failed to create action");
    e.to_string()
  })?;
  db.create_action_option_by_action(action.id, &action.pattern)
    .map_err(|e| e.to_string())?;
  RiboEvent::<()>::create_update_event(None, EventLabel::Action)
    .emit(&app)
    .map_err(|e| e.to_string())?;
  Ok(action)
}

#[tauri::command]
pub fn create_option<R: Runtime>(
  app: AppHandle<R>,
  option: NewRiboOption,
) -> CommandResult<RiboOption> {
  log::debug!("commands::action::create_option called");
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_option - failed to local db: {}",
      e
    );
    e.to_string()
  })?;

  let res = db.create_option(option).map_err(|e| {
    log::error!("commands::action::create_option - failed to create option");
    e.to_string()
  });

  RiboEvent::<()>::create_update_event(None, EventLabel::Option)
    .emit(&app)
    .map_err(|e| e.to_string())?;

  res
}

#[tauri::command]
pub fn create_action_option<R: Runtime>(
  app: AppHandle<R>,
  action: NewActionWithOption,
) -> CommandResult<ActionWithOption> {
  log::debug!("commands::action::create_action_option called");
  let state = app.state::<crate::store::db::Db>();
  let mut db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::create_action_option - failed to local db: {}",
      e
    );
    e.to_string()
  })?;
  log::debug!("create_action_option - action={:?}", action);
  let action_option = db.create_action_option(action.clone()).map_err(|e| {
    log::error!("commands::action::create_action_option - failed to create action option");
    e.to_string()
  })?;
  db.create_action_option_by_action(action_option.id, action.pattern.as_str())
  .map_err(|e| e.to_string())?;
  RiboEvent::<()>::create_update_event(None, EventLabel::ActionOption)
    .emit(&app)
    .map_err(|e| e.to_string())?;
  Ok(action_option)
}

#[tauri::command]
pub fn get_actions(state: State<'_, Db>) -> CommandResult<Vec<ActionWithOption>> {
  log::debug!("commands::action::get_actions called");
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::action::get_actions - failed to local db: {}", e);
    e.to_string()
  })?;

  db.get_action_options().map_err(|e| {
    log::error!("commands::action::get_action_options - failed to get actions");
    e.to_string()
  })
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
    Ok(None) => {
      log::error!("commands::action::get_action_by_id - failed to get action id={id}");
      Err(ribo_db::Error::NotFound(format!("{id}")).to_string())
    }
    Err(e) => {
      log::error!("commands::action::get_action_by_id - failed to get action id={id}");
      Err(e.to_string())
    }
  }
}

#[tauri::command]
pub fn get_options_by_action_id(state: State<'_, Db>, id: u64) -> CommandResult<Vec<RiboOption>> {
  log::debug!("commands::action::get_options_by_action_id called id={}", id);
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::action::get_options_by_action_id - failed to local db: {}", e);
    e.to_string()
  })?;
  db.get_options_by_action_id(id).map_err(|e| {
    log::error!("commands::action::get_options_by_action_id - failed to get options id={id}");
    e.to_string()
  })
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
        RiboEvent::<()>::create_update_event(None, EventLabel::ActionOption)
          .emit(&app)
          .map_err(|e| e.to_string())?;
        log::info!("commands::action::delete_action - success to delete action id={id}");
      }
      Ok(success)
    }
    Err(e) => {
      log::error!("commands::action::delete_action - failed to delete action id={id}");
      Err(e.to_string())
    }
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
  match db.delete_option(id) {
    Ok(success) => {
      if success {
        RiboEvent::<()>::create_update_event(None, EventLabel::Option)
          .emit(&app)
          .map_err(|e| e.to_string())?;
        log::info!("commands::action::delete_option - success to delete option id={id}");
      }
      Ok(success)
    }
    Err(e) => {
      log::error!("commands::action::delete_option - failed to delete option id={id}");
      Err(e.to_string())
    }
  }
}

#[tauri::command]
pub fn update_action<R: Runtime>(app: AppHandle<R>, action: UpdateAction) -> CommandResult<bool> {
  log::info!("commands::action::update_action called id={}", action.id);
  let state = app.state::<crate::store::db::Db>();
  let mut db = state.0.lock().map_err(|e| {
    log::error!(
      "commands::action::update_action - failed to local db: {}",
      e
    );
    e.to_string()
  })?;
  let id = action.id;
  let binding = action.clone();
  let pattern = binding.pattern.as_str();
  match db.update_action(action) {
    Ok(success) => {
      if success {
        db.update_action_option_by_action(id, pattern).map_err(|e| e.to_string())?;
        RiboEvent::<()>::create_update_event(None, EventLabel::Action)
          .emit(&app)
          .map_err(|e| e.to_string())?;
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
      Err(e.to_string())
    }
  }
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
  let id = option.id;
  match db.update_option(option) {
    Ok(success) => {
      if success {
        RiboEvent::<()>::create_update_event(None, EventLabel::Option)
          .emit(&app)
          .map_err(|e| e.to_string())?;
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
      Err(e.to_string())
    }
  }
}

#[tauri::command]
pub fn get_exec_action<R: Runtime>(
  app: AppHandle<R>,
  id: u64,
) -> CommandResult<Vec<ActionWithOption>> {
  log::info!("commands::action::exec_action called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let _db = state.0.lock().map_err(|e| {
    log::error!("commands::action::exec_action - failed to local db: {}", e);
    e.to_string()
  })?;
  todo!()

  // let data = db.get_record_by_id(id).map_err(|e| e.to_string())?;
  // match data {
  //   Some(record) => {
  //     log::info!(
  //       "commands::action::exec_action - success to exec action id={}",
  //       id
  //     );
  //     let action = match db.get_action_record_by_record(&record) {
  //       Ok(action) => action,
  //       Err(e) => {
  //         log::error!(
  //           "commands::action::exec_action - failed to get action record: {}",
  //           e
  //         );
  //         return Err(e.to_string());
  //       }
  //     };
  //     Ok(action)
  //   }
  //   None => {
  //     log::error!("commands::action::exec_action - record not found id={}", id);
  //     Ok(vec![])
  //   }
  // }
}

#[tauri::command]
pub fn exec_action<R: Runtime>(app: AppHandle<R>, id: u64) -> CommandResult<bool> {
  log::info!("commands::action::exec_action called id={}", id);
  let state = app.state::<crate::store::db::Db>();
  let db = state.0.lock().map_err(|e| {
    log::error!("commands::action::exec_action - failed to local db: {}", e);
    e.to_string()
  })?;

  let data = match db.get_action_by_id(id) {
    Ok(data) => data,
    Err(e) => {
      log::error!(
        "commands::action::exec_action - failed to get action: {}",
        e
      );
      return Err(e.to_string());
    }
  };
  log::debug!("暂未实现, {:?}", data);

  // TODO 暂未实现
  Ok(false)
}
