use serde_json::json;
use tauri::{AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tauri_plugin_store::StoreExt;

use crate::{
  events::RiboEvent,
  store::config::RiboConfig,
  utils::constant::{STORE_FILE, WIN_LABEL_TRAY_PANE},
};

#[tauri::command]
pub fn config_load<R: Runtime>(app: AppHandle<R>) -> Result<Option<RiboConfig>, String> {
  log::debug!("commands::config::config_load called");
  let store = app.store(STORE_FILE).map_err(|e| {
    log::error!(
      "commands::config::config_load - failed to open store: {}",
      e
    );
    e.to_string()
  })?;
  match store.get("config") {
    Some(config) => {
      let conf: RiboConfig = serde_json::from_value(config).map_err(|e| {
        log::error!("commands::config::config_load - parse error: {}", e);
        e.to_string()
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
pub fn config_save<R: Runtime>(app: AppHandle<R>, config: RiboConfig) -> Result<(), String> {
  log::info!("保存配置 => {:?}", config);
  let store = app.store(STORE_FILE).map_err(|e| {
    log::error!("保存配置失败 => {:?}", e);
    e.to_string()
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
  log::info!("通知前端更新状态");
  RiboEvent::<()>::create_refresh_event(None, WIN_LABEL_TRAY_PANE)
    .emit(&app)
    .map_err(|e| {
      log::error!("保存配置 -> 通知前端更新状态失败 => {e:?}");
      e.to_string()
    })?;
  Ok(())
}

pub(crate) fn autostart_setting<R: Runtime>(app: &AppHandle<R>, config: &RiboConfig) {
  let autostart_manager = app.autolaunch();
  match (autostart_manager.is_enabled(), config.get_autostart()) {
    (Ok(false), true) => match autostart_manager.enable() {
      Ok(_) => {
        log::info!("开机自启已启用");
      }
      Err(e) => {
        log::error!("启用开机自启失败 => {:?}", e);
      }
    },
    (Ok(true), false) => match autostart_manager.disable() {
      Ok(_) => {
        log::info!("开机自启已禁用");
      }
      Err(e) => {
        log::error!("禁用开机禁用失败 => {:?}", e);
      }
    },
    _ => {}
  }
}

pub(crate) fn shortcut_setting<R: Runtime>(
  app: &AppHandle<R>,
  config: &RiboConfig,
  old_config: Option<RiboConfig>,
) -> anyhow::Result<()> {
  let shortcut = app.global_shortcut();
  if let Some(old_config) = old_config {
    if let Some(k) = &old_config.hotkey.clear {
      let clear_key: Shortcut = k.try_into()?;
      shortcut.unregister(clear_key)?;
      log::info!("已注销旧的清理快捷键 => {:?}", clear_key);
    }
    if let Some(k) = &old_config.hotkey.pane {
      let pane_key: Shortcut = k.try_into()?;
      shortcut.unregister(pane_key)?;
      log::info!("已注销旧的面板快捷键 => {:?}", pane_key);
    }
  }

  if let Some(k) = &config.hotkey.clear {
    let clear_key: Shortcut = k.try_into()?;
    shortcut.register(clear_key)?;
    log::info!("已注册清理快捷键 => {:?}", clear_key);
  }
  if let Some(k) = &config.hotkey.pane {
    let pane_key: Shortcut = k.try_into()?;
    shortcut.register(pane_key)?;
    log::info!("已注册面板快捷键 => {:?}", pane_key);
  }

  Ok(())
}
