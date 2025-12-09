use ribo_db::models::{NewHistory, QueryHistory, UpdateHistory};
use tauri::{AppHandle, Runtime, State};
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
pub fn create_data<R: Runtime>(
  app: AppHandle<R>,
  state: State<'_, Db>,
  data: NewHistory,
) -> Result<(), String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  let config = super::store::store_load(app)?;
  match config {
    Some(config) => match config.general {
      Some(general) => {
        db.create_data(data, general.max)
          .map_err(|e| e.to_string())?;
      }
      None => {
        db.create_data(data, None).map_err(|e| e.to_string())?;
      }
    },
    None => {
      db.create_data(data, None).map_err(|e| e.to_string())?;
    }
  }
  // 通知刷新
  Ok(())
}

#[tauri::command]
pub fn update_data(state: State<'_, Db>, data: UpdateHistory) -> Result<(), String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  db.update_data(data.clone()).map_err(|e| e.to_string())?;
  log::info!("更新 {} 成功", data.id);
  // 通知刷新
  Ok(())
}

#[tauri::command]
pub fn delete_data(state: State<'_, Db>, id: usize) -> Result<(), String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  db.delete_data(id).map_err(|e| e.to_string())?;
  log::info!("删除 {id} 成功");
  // 通知刷新
  Ok(())
}

#[tauri::command]
pub fn query_data(state: State<'_, Db>) -> Result<QueryHistory, String> {
  let db = state.0.lock().map_err(|e| e.to_string())?;
  let data = db.query_data().map_err(|e| e.to_string())?;
  log::info!("总条数 {}", data.total);
  Ok(data)
}
