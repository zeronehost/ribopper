use tauri::Manager;
use tauri_plugin_store::StoreExt;

use crate::{store::config::RiboConfig, utils::constant::STORE_FILE};

mod commands;
mod events;
mod logger;
mod models;
mod store;
mod tray;
mod utils;
mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let ctx = tauri::generate_context!();
  let mut builder = tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
    .plugin(
      tauri_plugin_log::Builder::new()
        .targets(crate::logger::targets())
        .level(crate::logger::level())
        .with_colors(tauri_plugin_log::fern::colors::ColoredLevelConfig::default())
        .max_file_size(crate::logger::MAX_FILE_SIZE)
        .format(|o, m, r| {
          o.finish(format_args!(
            "{}[{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            r.level(),
            m
          ))
        })
        .build(),
    );
  #[cfg(desktop)]
  {
    builder = builder
      .plugin(tauri_plugin_autostart::init(
        tauri_plugin_autostart::MacosLauncher::AppleScript,
        None,
      ))
      .plugin(
        tauri_plugin_global_shortcut::Builder::new()
          .with_handler(move |app, sc, ev| {
            match crate::commands::config::config_load(app.clone()) {
              Ok(config) => match config {
                Some(conf) => {
                  if let Some(k) = conf.hotkey.clear {
                    use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};

                    if let Ok(key) = TryInto::<Shortcut>::try_into(&k)
                      && sc == &key
                    {
                      log::info!("global shortcut: clear shortcut triggered");
                      if ev.state() == ShortcutState::Pressed {
                        match crate::commands::record::clear_records(app.clone()) {
                          Ok(_) => {
                            log::info!("global shortcut: clear records success");
                          }
                          Err(e) => {
                            log::error!("global shortcut: clear records failed: {}", e);
                          }
                        }
                      }
                    }
                  }
                  if let Some(k) = conf.hotkey.pane {
                    use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};

                    if let Ok(key) = TryInto::<Shortcut>::try_into(&k)
                      && sc == &key
                    {
                      log::info!("global shortcut: clear shortcut triggered");
                      if ev.state() == ShortcutState::Pressed {
                        use tauri_plugin_dialog::DialogExt;

                        log::info!("global shortcut: toggle tray pane window");
                        app
                          .clone()
                          .dialog()
                          .message("暂不支持打开上下文菜单，请点击系统托盘图标打开")
                          .title("温馨提示")
                          .show(|_| {});
                      }
                    }
                  }
                }
                None => {
                  log::warn!("global shortcut: config not exist");
                }
              },
              Err(e) => {
                log::error!("global shortcut: Failed to load config: {}", e);
              }
            }
          })
          .build(),
      );
  }

  builder = builder
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_store::Builder::new().build());

  let app = builder
    .invoke_handler(tauri::generate_handler![
      crate::commands::window::close_window,
      crate::commands::window::web_log,
      //-------------------------------------------
      // db commands
      crate::commands::record::get_records,
      crate::commands::record::get_record,
      crate::commands::record::delete_record,
      crate::commands::record::create_record,
      crate::commands::record::update_record,
      crate::commands::record::clear_records,
      crate::commands::record::copy_record,
      crate::commands::record::qrcode_record,
      // crate::commands::target::get_targets,
      // crate::commands::target::create_target,
      // crate::commands::target::delete_target,
      //-------------------------------------------
      // store commands
      crate::commands::config::config_load,
      crate::commands::config::config_save,
      //-------------------------------------------
      // cmd commands
      crate::commands::action::get_actions,
      crate::commands::action::get_action_by_id,
      crate::commands::action::create_action,
      crate::commands::action::create_action_option,
      crate::commands::action::create_option,
      crate::commands::action::delete_action,
      crate::commands::action::delete_option,
      crate::commands::action::update_action,
      crate::commands::action::update_option,
      crate::commands::action::get_exec_action,
      crate::commands::action::exec_action,
      //-------------------------------------------
    ])
    .setup(|app| {
      crate::store::Store::init(app.handle())?;
      crate::tray::Tray::init(app.handle())?;

      let store = app.store(STORE_FILE)?;
      if let Some(config) = store.get("config") {
        let config: RiboConfig = serde_json::from_value(config).unwrap_or_default();
        crate::commands::config::autostart_setting(app.handle(), &config);
        crate::commands::config::shortcut_setting(app.handle(), &config, None)?;
      }
      Ok(())
    })
    .build(ctx)
    .expect("error while running tauri application");

  app.run(|app, event| {
    if let tauri::RunEvent::WindowEvent {
      label,
      event: win_e,
      ..
    } = event
      && let tauri::WindowEvent::CloseRequested { api, .. } = win_e
    {
      log::info!("Window close requested for label={}", label);
      let w = app.get_webview_window(label.as_str()).unwrap();
      w.hide().unwrap();
      log::info!("Window hidden; prevented close for label={}", label);
      api.prevent_close();
    }
  })
}
