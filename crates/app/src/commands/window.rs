use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub fn close_window<R: Runtime>(app: AppHandle<R>, label: &str) -> Result<(), String> {
  log::info!("关闭Label为：{label}的窗口");
  match app.get_webview_window(label) {
    Some(window) => match window.close() {
      Ok(_) => {}
      Err(e) => {
        log::error!("Failed to close window: {}", e);
        return Err(e.to_string());
      }
    },
    None => {}
  };
  Ok(())
}
