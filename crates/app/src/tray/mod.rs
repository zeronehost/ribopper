use tauri::{
  AppHandle, Runtime,
  menu::{MenuBuilder, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use crate::utils::constant::APP_NAME;

pub(crate) struct Tray;

impl Tray {
  pub fn init<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<()> {
    // let pkg_info = app.package_info();
    #[allow(unused_mut)]
    let mut menu = MenuBuilder::new(app);
    #[cfg(target_os = "linux")]
    {
      menu = menu
        .item(&MenuItem::with_id(
          app,
          "main",
          "剪贴板",
          true,
          None::<&str>,
        )?)
        .separator();
    }
    let menu = menu
      .item(&MenuItem::with_id(
        app,
        "clear",
        "清空历史记录",
        true,
        None::<&str>,
      )?)
      .item(&MenuItem::with_id(
        app,
        "setting",
        "设置",
        true,
        None::<&str>,
      )?)
      .item(&MenuItem::with_id(
        app,
        "about",
        "关于",
        true,
        None::<&str>,
      )?)
      .separator()
      .item(&MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?)
      .build()?;

    TrayIconBuilder::new()
      .icon(app.default_window_icon().unwrap().clone())
      .menu(&menu)
      .on_tray_icon_event(|icon, ev| {
        if let TrayIconEvent::Click {
          button: MouseButton::Left,
          button_state: MouseButtonState::Down,
          ..
        } = ev
        {
          log::info!("左键点击托盘图标");
          let app = icon.app_handle();
          crate::window::open_tray_pane(app).unwrap();
        }
      })
      .on_menu_event(|app, ev| match ev.id().as_ref() {
        "main" => {
          log::info!("打开主窗口");
          crate::window::open_tray_pane(app).unwrap();
        }
        "quit" => {
          log::info!("确认退出应用");
          let app_handle = app.clone();
          app
            .dialog()
            .message("确认要退出应用？")
            .title("温馨提示")
            .buttons(MessageDialogButtons::OkCancelCustom(
              "退出".to_string(),
              "取消".to_string(),
            ))
            .show(move |result| {
              if result {
                log::info!("退出应用");
                log::info!("================================");
                app_handle.exit(0);
              }
            });
        }
        "clear" => {
          log::info!("清空历史记录");
          crate::commands::record::clear_records(app.clone()).unwrap();
        }
        "setting" => {
          log::info!("打开设置窗口");
          crate::window::open_setting_window(app).unwrap();
        }
        "about" => {
          log::info!("打开关于");
          let pkg_info = app.package_info();
          app
            .dialog()
            .message(format!("{APP_NAME}\n版本: {}", pkg_info.version))
            .kind(MessageDialogKind::Info)
            .title("关于")
            .buttons(MessageDialogButtons::Ok)
            .show(|_| {});
        }
        _ => {}
      })
      .show_menu_on_left_click(false)
      .build(app)?;
    Ok(())
  }
}
