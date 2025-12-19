use image::EncodableLayout;
use tauri::Manager;

use crate::utils::{constant::RIBO_SCHEME, qrcode::create_qrcode};

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
  let mut builder = tauri::Builder::default();

  builder = builder
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
    )
    .plugin(tauri_plugin_autostart::Builder::new().build())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .register_uri_scheme_protocol(RIBO_SCHEME, |ctx, req| {
      let path = req.uri().path();
      if path == "/qrcode" {
        let id = req.uri().query().unwrap();
        let id = id.parse::<u64>().unwrap();
        let app = ctx.app_handle();
        let state = app.try_state::<crate::store::db::Db>().unwrap();
        let record = crate::commands::record::get_record(state, id).unwrap();
        match record.typ {
          ribo_db::models::RecordType::Text => {
            return tauri::http::Response::builder()
              .status(tauri::http::StatusCode::OK)
              .header(tauri::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
              .header(tauri::http::header::CACHE_CONTROL, "application/octet-stream")
              .body(create_qrcode(record.text.unwrap_or_default().as_bytes()).unwrap_or(
                create_qrcode(b"text too long".as_bytes()).unwrap()
              ))
              .unwrap();
          }
          ribo_db::models::RecordType::Image => {
            return tauri::http::Response::builder()
              .header(tauri::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
              .header(tauri::http::header::CACHE_CONTROL, "application/octet-stream")
              .status(tauri::http::StatusCode::OK)
              .body(create_qrcode(record.image.unwrap_or_default()).unwrap())
              .unwrap();
          }
          ribo_db::models::RecordType::Files => {
            return tauri::http::Response::builder()
              .header(tauri::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
              .header(tauri::http::header::CACHE_CONTROL, "application/octet-stream")
              .status(tauri::http::StatusCode::OK)
              .body(
                create_qrcode(
                  record
                    .files
                    .unwrap_or_default()
                    .iter()
                    .map(|i| i.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
                    .as_bytes(),
                )
                .unwrap(),
              )
              .unwrap();
          }
        }
      }
      tauri::http::Response::builder()
        .header(tauri::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(tauri::http::StatusCode::BAD_REQUEST)
        .body(b"".to_vec())
        .unwrap()
    });

  let app = builder
    .invoke_handler(tauri::generate_handler![
      crate::commands::window::close_window,
      //-------------------------------------------
      // db commands
      crate::commands::record::get_records,
      crate::commands::record::get_record,
      crate::commands::record::delete_record,
      crate::commands::record::create_record,
      crate::commands::record::update_record,
      crate::commands::record::clear_records,
      crate::commands::record::copy_record,
      // crate::commands::record::qrcode_record,
      crate::commands::target::get_targets,
      crate::commands::target::create_target,
      crate::commands::target::delete_target,
      //-------------------------------------------
      // store commands
      crate::commands::config::config_load,
      crate::commands::config::config_save,
      //-------------------------------------------
    ])
    .setup(|app| {
      crate::store::Store::init(app.handle())?;
      crate::tray::Tray::init(app.handle())?;
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
      let w = app.get_webview_window(label.as_str()).unwrap();
      w.hide().unwrap();
      api.prevent_close();
    }
  })
}
