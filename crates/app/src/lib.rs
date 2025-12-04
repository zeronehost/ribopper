use tauri::Manager;

mod commands;
mod logger;
mod tray;
mod utils;
mod window;
mod store;
mod models;

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
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_store::Builder::new().build());

  let app = builder
    .invoke_handler(tauri::generate_handler![
      crate::commands::window::close_window,
      crate::commands::db::clear_data,
      //-------------------------------------------
      // store commands
      crate::commands::store::store_load,
      crate::commands::store::store_set_theme,
      //-------------------------------------------
    ])
    .setup(|app| {
      crate::store::Store::init(app.handle())?;

      crate::tray::Tray::init(app.handle())?;
      Ok(())
    })
    .build(ctx)
    .expect("error while running tauri application");

  app.run(|app, event| match event {
    tauri::RunEvent::WindowEvent {
      label,
      event: win_e,
      ..
    } => match win_e {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        let w = app.get_webview_window(label.as_str()).unwrap();
        w.hide().unwrap();
        api.prevent_close();
      }
      // tauri::WindowEvent::Moved(_pos) => {
      //   crate::window::set_window_pos(app, label.as_str()).unwrap();
      // }
      _ => {}
    },
    _ => {}
  })
}
