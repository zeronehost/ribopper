use tauri::{AppHandle, Runtime};
#[cfg(not(debug_assertions))]
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
#[cfg(not(debug_assertions))]
use tauri_plugin_updater::UpdaterExt;

use crate::{
  models::{AppInfo, Features},
  utils::error::Result,
};

#[tauri::command]
pub fn get_app_info<R: Runtime>(app: AppHandle<R>) -> Result<AppInfo> {
  let info = app.package_info();

  Ok(AppInfo {
    name: info.name.clone(),
    version: info.version.to_string(),
    authors: info.authors.to_string(),
    description: info.description.to_string(),
    license: env!("CARGO_PKG_LICENSE").to_string(),
    website: env!("CARGO_PKG_HOMEPAGE").to_string(),
    features: Features::default(),
  })
}

// TODO 优化更新逻辑
#[tauri::command]
pub async fn update<R: Runtime>(app: AppHandle<R>, channel: tauri::ipc::Channel) -> Result<()> {
  check_update(app, Some(channel)).await?;
  Ok(())
}

pub(crate) async fn check_update<R: Runtime>(
  #[allow(unused)]
  app: AppHandle<R>,
  #[allow(unused)]
  channel: Option<tauri::ipc::Channel>,
) -> Result<()> {
  log::info!("commands::common::check_update called");
  #[cfg(not(debug_assertions))]
  if let Some(update) = app.updater()?.check().await? {
    app
      .dialog()
      .message("存在新版本，是否更新？")
      .title("应用更新")
      .buttons(MessageDialogButtons::OkCancelCustom(
        "立即更新".to_string(),
        "稍后更新".to_string(),
      ))
      .show(move |res| {
        if res {
          tauri::async_runtime::block_on(async move {
            let mut downloaded = 0;

            update
              .download_and_install(
                |chunk_length, content_length| {
                  downloaded += chunk_length;
                  if let Some(size) = content_length {
                    let progress = downloaded as f64 / size as f64 * 100.0;
                    if let Some(channel) = &channel {
                      let payload = serde_json::json!({
                        "progress": progress,
                        "total": size,
                        "downloaded": downloaded,
                        "indeterminate": true,
                        "done": false
                      });
                      channel.send(payload.to_string().into()).unwrap();
                    }
                    log::info!("download progress: {:.2}%", progress);
                  } else {
                    log::info!("download progress: {}", downloaded);
                    if let Some(channel) = &channel {
                      let payload = serde_json::json!({
                        "downloaded": downloaded,
                        "done": false,
                        "indeterminate": false,
                      });
                      channel.send(payload.to_string().into()).unwrap();
                    }
                  }
                },
                || {
                  log::info!("download finished");
                  if let Some(channel) = &channel {
                    let payload = serde_json::json!({
                      "done": true,
                      "progress": 100.0,
                      "indeterminate": true,
                    });
                    channel.send(payload.to_string().into()).unwrap();
                  }
                },
              )
              .await
              .unwrap();
            log::info!("update installed");
            app
              .dialog()
              .message("下载完成，立即安装")
              .kind(MessageDialogKind::Info)
              .title("应用更新")
              .buttons(MessageDialogButtons::OkCancelCustom(
                "立即安装".to_string(),
                "稍后安装".to_string(),
              ))
              .show(move |result| {
                if result {
                  let _ = app.restart();
                }
              });
          })
        }
      });
  }

  Ok(())
}
