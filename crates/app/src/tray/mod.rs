use tauri::{
  menu::{MenuBuilder, MenuItem, PredefinedMenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  AppHandle, Runtime,
};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

pub(crate) struct Tray;

impl Tray {
  pub fn init<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<()> {
    TrayIconBuilder::new()
      .icon(app.default_window_icon().unwrap().clone())
      .menu(
        &MenuBuilder::new(app)
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
          .item(&PredefinedMenuItem::about(app, Some("关于"), None)?)
          .separator()
          .item(&MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?)
          .build()?,
      )
      .on_menu_event(|app, ev| match ev.id().as_ref() {
        "quit" => {
          log::info!("退出应用");
          let answer = app
            .dialog()
            .message("确认要退出应用？")
            .title("温馨提示")
            .buttons(MessageDialogButtons::OkCancelCustom(
              "退出".to_string(),
              "取消".to_string(),
            ))
            .blocking_show();
          if answer {
            log::info!("确认退出应用");
            app.exit(0);
          }
        }
        "clear" => {
          log::info!("清空历史记录");
          let answer = app
            .dialog()
            .message("确认要清空历史记录？")
            .title("温馨提示")
            .buttons(MessageDialogButtons::OkCancelCustom(
              "确认".to_string(),
              "取消".to_string(),
            ))
            .blocking_show();
          if answer {
            log::info!("确认清空历史记录");
            todo!("暂未实现");
          }
        }
        "setting" => {
          log::info!("打开设置窗口");
          todo!("暂未实现");
        }
        _ => {}
      })
      .on_tray_icon_event(|_icon, ev| match ev {
        TrayIconEvent::Click {
          button,
          button_state,
          ..
        } => match (button, button_state) {
          (MouseButton::Left, MouseButtonState::Down) => {
            log::info!("左键点击托盘图标");
          }
          _ => {}
        },
        _ => {}
      })
      .show_menu_on_left_click(false)
      .build(app)?;
    Ok(())
  }
}
