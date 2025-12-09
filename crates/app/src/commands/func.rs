use ribo_db::models::HistoryType;
use tauri::{AppHandle, Runtime, State};
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::store::db::Db;


#[tauri::command]
pub fn copy_data<R: Runtime>(app: AppHandle<R>, state: State<'_, Db>, id: usize) -> Result<(), String> {
  log::info!("复制数据: {id}");
  let db = state.0.lock().map_err(|e| e.to_string())?;
  let data = db.query_data_by_id(id).map_err(|e| e.to_string())?;
  let clipboard = app.clipboard();
  match data.typ {
    HistoryType::Text => clipboard.write_text(data.content),
    _ => { todo!("暂未实现") }
    // HistoryType::File => clipboard.write_text(data.content),
    // HistoryType::Dir =>  clipboard.write_text(data.content),
    // HistoryType::Image => clipboard.write_text(data.content),
  }.map_err(|e| e.to_string())
}