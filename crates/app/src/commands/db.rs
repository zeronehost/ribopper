use tauri::{AppHandle, Runtime};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

#[tauri::command]
pub async fn clear_data<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
  log::info!("清楚历史记录");
  let answer = app
    .dialog()
    .message("确认要清空历史记录？")
    .title("温馨提示")
    .buttons(MessageDialogButtons::OkCancelCustom(
      "确定".to_string(),
      "取消".to_string(),
    ))
    .blocking_show();
  if answer {
    log::info!("确认清空历史记录");
    log::debug!("暂未实现");
  }
  Ok(())
}
