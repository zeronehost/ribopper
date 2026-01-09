use tauri::{
  AppHandle, Runtime,
  menu::{MenuBuilder, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use crate::{commands::config::config_load, utils::constant::APP_NAME};

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
          log::info!("tray::on_tray_icon_event::click");
          let app = icon.app_handle();
          crate::window::open_tray_pane(app).unwrap();
        }
      })
      .on_menu_event(|app, ev| match ev.id().as_ref() {
        "main" => {
          log::info!("tray::on_menu_event::main open");
          crate::window::open_tray_pane(app).unwrap();
        }
        "quit" => {
          log::info!("tray::on_menu_event::quit");
          let app_handle = app.clone();
          let exit_confirm = if let Ok(Some(config)) = config_load(app_handle.clone()) {
            config.get_exit_confirm()
          } else {
            true
          };
          if !exit_confirm {
            log::info!("tray::on_menu_event::quit::exit");
            log::info!("================================");
            app_handle.exit(0);
          }
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
                log::info!("tray::on_menu_event::quit::exit");
                log::info!("================================");
                app_handle.exit(0);
              }
            });
        }
        "clear" => {
          log::info!("tray::on_menu_event::clear");
          crate::commands::record::clear_records(app.clone()).unwrap();
        }
        "setting" => {
          log::info!("tray::on_menu_event::setting");
          crate::window::open_setting_window(app).unwrap();
        }
        "about" => {
          log::info!("tray::on_menu_event::about");
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
