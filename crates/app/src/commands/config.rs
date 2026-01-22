use serde_json::json;
use tauri::{AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tauri_plugin_store::StoreExt;

use crate::{
  events::{EventAction, EventLabel, RiboEvent},
  store::config::RiboConfig,
  utils::{constant::STORE_FILE, error::Result},
};

#[tauri::command]
pub fn config_load<R: Runtime>(app: AppHandle<R>) -> Result<Option<RiboConfig>> {
  log::debug!("commands::config::config_load called");
  let store = app.store(STORE_FILE).map_err(|e| {
    log::error!(
      "commands::config::config_load - failed to open store: {}",
      e
    );
    e
  })?;
  match store.get("config") {
    Some(config) => {
      let conf: RiboConfig = serde_json::from_value(config).map_err(|e| {
        log::error!("commands::config::config_load - parse error: {}", e);
        e
      })?;

      log::debug!("commands::config::config_load - returning config");
      Ok(Some(conf))
    }
    None => {
      log::debug!("commands::config::config_load - no config found");
      Ok(None)
    }
  }
}

#[tauri::command]
pub fn config_save<R: Runtime>(app: AppHandle<R>, config: RiboConfig) -> Result<()> {
  log::info!("commands::config::config_save called");
  let store = app.store(STORE_FILE).map_err(|e| {
    log::error!("commands::config::config_save - failed to open store: {e}");
    e
  })?;

  autostart_setting(&app, &config);
  let _ = shortcut_setting(
    &app,
    &config,
    store
      .get("config")
      .map(|c| serde_json::from_value::<RiboConfig>(c).unwrap_or_default()),
  );

  store.set("config", json!(config));
  log::info!("commands::config::config_save - config saved");
  RiboEvent::create_update_event(EventLabel::Config, EventAction::Save).emit(&app)?;
  Ok(())
}

pub(crate) fn autostart_setting<R: Runtime>(app: &AppHandle<R>, config: &RiboConfig) {
  let autostart_manager = app.autolaunch();
  match (autostart_manager.is_enabled(), config.get_autostart()) {
    (Ok(false), true) => match autostart_manager.enable() {
      Ok(_) => {
        log::info!("commands::config::autostart_setting - autostart enabled");
      }
      Err(e) => {
        log::error!(
          "commands::config::autostart_setting - enable autostart failed => {:?}",
          e
        );
      }
    },
    (Ok(true), false) => match autostart_manager.disable() {
      Ok(_) => {
        log::info!("commands::config::autostart_setting - autostart disabled");
      }
      Err(e) => {
        log::error!(
          "commands::config::autostart_setting - disable autostart failed => {:?}",
          e
        );
      }
    },
    _ => {}
  }
}

pub(crate) fn shortcut_setting<R: Runtime>(
  app: &AppHandle<R>,
  config: &RiboConfig,
  old_config: Option<RiboConfig>,
) -> Result<()> {
  log::info!("commands::config::shortcut_setting called");
  let shortcut = app.global_shortcut();
  if let Some(old_config) = old_config {
    if let Some(k) = &old_config.hotkey.clear {
      let clear_key: Shortcut = k.try_into()?;
      shortcut.unregister(clear_key)?;
      log::info!(
        "commands::config::shortcut_setting - unregistered old clear key => {:?}",
        clear_key
      );
    }
    if let Some(k) = &old_config.hotkey.pane {
      let pane_key: Shortcut = k.try_into()?;
      shortcut.unregister(pane_key)?;
      log::info!(
        "commands::config::shortcut_setting - unregistered old pane key => {:?}",
        pane_key
      );
    }
  }

  if let Some(k) = &config.hotkey.clear {
    let clear_key: Shortcut = k.try_into()?;
    shortcut.register(clear_key)?;
    log::info!(
      "commands::config::shortcut_setting - registered clear key => {:?}",
      clear_key
    );
  }
  if let Some(k) = &config.hotkey.pane {
    let pane_key: Shortcut = k.try_into()?;
    shortcut.register(pane_key)?;
    log::info!(
      "commands::config::shortcut_setting - registered pane key => {:?}",
      pane_key
    );
  }

  Ok(())
}
