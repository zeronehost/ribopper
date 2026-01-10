use tauri::{AppHandle, Runtime};

use crate::models::AppInfo;

use super::CommandResult;

#[tauri::command]
pub fn get_app_info<R: Runtime>(app: AppHandle<R>) -> CommandResult<AppInfo> {
  let info = app.package_info();

  Ok(AppInfo {
    name: info.name.clone(),
    version: info.version.to_string(),
    authors: info.authors.to_string(),
    description: info.description.to_string(),
    license: env!("CARGO_PKG_LICENSE").to_string(),
    website: env!("CARGO_PKG_HOMEPAGE").to_string()
  })
}