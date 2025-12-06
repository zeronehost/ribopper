pub(crate) fn targets() -> Vec<tauri_plugin_log::Target> {
  if cfg!(debug_assertions) {
    vec![
      tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
      // tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
      //   file_name: Some(format!(
      //     "ribopper-_dev-{}",
      //     chrono::Local::now().format("%Y-%m-%d")
      //   )),
      // }),
      tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
    ]
  } else {
    vec![
      tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
        file_name: Some(format!(
          "ribopper-{}",
          chrono::Local::now().format("%Y-%m-%d")
        )),
      }),
      tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
    ]
  }
}

pub(crate) fn level() -> log::LevelFilter {
  if cfg!(debug_assertions) {
    log::LevelFilter::Debug
  } else {
    log::LevelFilter::Info
  }
}

pub(crate) const MAX_FILE_SIZE: u128 = 1024 * 1024 * 10; // 10MB
