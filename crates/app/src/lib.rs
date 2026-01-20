use tauri::Manager;
use tauri_plugin_store::StoreExt;

#[cfg(not(debug_assertions))]
use crate::commands::common::check_update;
use crate::{store::config::RiboConfig, utils::constant::STORE_FILE};

mod commands;
mod events;
mod logger;
mod menu;
mod models;
mod store;
mod utils;
mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let ctx = tauri::generate_context!();
  let mut builder = tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
    .plugin(tauri_plugin_updater::Builder::new().build())
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
                        // use tauri_plugin_dialog::DialogExt;

                        // log::info!("global shortcut: toggle tray pane window");
                        // app
                        //   .clone()
                        //   .dialog()
                        //   .message("暂不支持打开上下文菜单，请点击系统托盘图标打开")
                        //   .title("温馨提示")
                        //   .show(|_| {});
                        if let Err(e) = crate::window::open_context_pane(&app) {
                          log::error!("global shortcut: open context pane failed: {}", e);
                        }
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
  #[cfg(feature = "action")]
  {
    builder = builder.plugin(tauri_plugin_opener::init());
  }

  builder = builder
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_store::Builder::new().build());

  let app = builder
    .register_asynchronous_uri_scheme_protocol("ribopper", |ctx, request, responder| {
      let p = request.uri().path()[1..].to_string();
      log::info!("ribopper: request image: {}", p);
      let app = ctx.app_handle();
      let root = match crate::utils::path::get_images_path(app) {
        Ok(root) => root,
        Err(e) => {
          log::error!("ribopper: get images path failed: {}", e);
          return responder.respond(
            tauri::http::Response::builder()
              .status(tauri::http::StatusCode::INTERNAL_SERVER_ERROR)
              .body("服务器内部错误".as_bytes().to_vec())
              .unwrap(),
          );
        }
      };
      tauri::async_runtime::spawn(async move {
        let path = root.join(p);
        if path.exists() {
          if let Ok(data) = std::fs::read(&path) {
            responder.respond(
              tauri::http::Response::builder()
                .status(200)
                .body(data)
                .unwrap(),
            );
          } else {
            log::error!("ribopper: read file failed: {}", &path.display());
            responder.respond(
              tauri::http::Response::builder()
                .status(tauri::http::StatusCode::BAD_REQUEST)
                .header(tauri::http::header::CONTENT_TYPE, "text/plain")
                .body("failed to read file".as_bytes().to_vec())
                .unwrap(),
            )
          }
        }
      });
    })
    .invoke_handler(tauri::generate_handler![
      crate::commands::window::close_window,
      crate::commands::window::web_log,
      //-------------------------------------------
      // record commands
      crate::commands::record::get_records,
      crate::commands::record::get_record,
      crate::commands::record::delete_record,
      crate::commands::record::create_record,
      crate::commands::record::update_record,
      crate::commands::record::clear_records,
      crate::commands::record::copy_record,
      crate::commands::record::qrcode_record,
      //-------------------------------------------
      // store commands
      crate::commands::config::config_load,
      crate::commands::config::config_save,
      //-------------------------------------------
      // action commands
      #[cfg(feature = "action")]
      crate::commands::action::get_actions,
      #[cfg(feature = "action")]
      crate::commands::action::get_action_by_id,
      #[cfg(feature = "action")]
      crate::commands::action::get_options_by_action_id,
      #[cfg(feature = "action")]
      crate::commands::action::create_action,
      #[cfg(feature = "action")]
      crate::commands::action::create_action_option,
      #[cfg(feature = "action")]
      crate::commands::action::create_option,
      #[cfg(feature = "action")]
      crate::commands::action::delete_action,
      #[cfg(feature = "action")]
      crate::commands::action::delete_option,
      #[cfg(feature = "action")]
      crate::commands::action::update_action,
      #[cfg(feature = "action")]
      crate::commands::action::update_option,
      #[cfg(feature = "action")]
      crate::commands::action::show_record_actions,
      //-------------------------------------------
      // common
      crate::commands::common::get_app_info,
      crate::commands::common::update,
      //-------------------------------------------
    ])
    .setup(|app| {
      crate::store::Store::init(app.handle())?;
      crate::menu::Tray::init(app.handle())?;

      let store = app.store(STORE_FILE)?;
      if let Some(config) = store.get("config") {
        let config: RiboConfig = serde_json::from_value(config).unwrap_or_default();
        crate::commands::config::autostart_setting(app.handle(), &config);
        crate::commands::config::shortcut_setting(app.handle(), &config, None)?;
      }
      // 启动时检查更新
      #[cfg(not(debug_assertions))]
      {
        let app_handle = app.handle().clone();
        tauri::async_runtime::spawn(async move {
          let _ = check_update(app_handle, None).await;
        });
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
