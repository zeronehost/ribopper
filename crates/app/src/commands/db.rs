use ribo_db::models::{History, NewHistory, UpdateHistory};
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

use crate::store::db::Db;

#[tauri::command]
pub async fn clear_data<R: Runtime>(app: AppHandle<R>, state: State<'_, Db>) -> Result<(), String> {
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
    let db = state.0.lock().map_err(|e| e.to_string())?;
    db.clear_data().map_err(|e| e.to_string())?;
    // 通知刷新
  }
  Ok(())
}

#[tauri::command]
pub async fn create_data(state: State<'_, Db>, data: NewHistory) -> Result<(), String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  db.create_data(data).map_err(|e| e.to_string())?;
  // 通知刷新
  Ok(())
}

#[tauri::command]
pub async fn update_data(state: State<'_, Db>, data: UpdateHistory) -> Result<(), String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  db.update_data(data).map_err(|e| e.to_string())?;
  // 通知刷新
  Ok(())
}

#[tauri::command]
pub async fn delete_data(state: State<'_, Db>, id: usize) -> Result<(), String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  db.delete_data(id).map_err(|e| e.to_string())?;
  // 通知刷新
  Ok(())
}

#[tauri::command]
pub async fn query_data(
  state: State<'_, Db>,
  index: usize,
  size: usize,
) -> Result<Vec<History>, String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  let data = db.query_data(index, size).map_err(|e| e.to_string())?;
  Ok(data)
}

#[tauri::command]
pub async fn query_total(state: State<'_, Db>) -> Result<usize, String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  let data = db.query_total().map_err(|e| e.to_string())?;
  Ok(data)
}
