#[cfg(debug_assertions)]
use crate::utils::constant::WIN_URL_TRAY_PANE;
use crate::utils::{
  constant::{WIN_LABEL_MAIN, WIN_LABEL_TRAY_PANE, WIN_NANE, WIN_URL_SETTING},
  pos::calc_pane_pos,
};
use serde_json::json;
use tauri::{AppHandle, Emitter, Manager, Runtime};

pub fn open_setting_window<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
  log::info!("打开设置窗口");
  match app.get_webview_window(WIN_LABEL_MAIN) {
    Some(win) => match win.is_visible() {
      Ok(true) => win.hide(),
      Ok(false) => {
        log::info!("设置窗口存在且是隐藏状态");
        win.show()?;
        win.set_focus()
      }
      _ => Ok(()),
    },
    None => {
      log::info!("设置窗口不存在");
      let win = tauri::WebviewWindowBuilder::new(
        app,
        WIN_LABEL_MAIN,
        #[cfg(debug_assertions)]
        tauri::WebviewUrl::External(
          app
            .config()
            .build
            .dev_url
            .clone()
            .unwrap()
            .join(WIN_URL_SETTING)
            .unwrap(),
        ),
        #[cfg(not(debug_assertions))]
        tauri::WebviewUrl::Url(format!("index.html{}", WIN_URL_SETTING)),
      )
      .visible(false)
      .inner_size(800., 600.)
      .center()
      .title(WIN_NANE)
      .build()?;
      win.show()?;
      win.set_focus()?;
      Ok(())
    }
  }
}

pub fn open_tray_pane<R: Runtime>(
  app: &AppHandle<R>,
  pos: tauri::PhysicalPosition<f64>,
) -> tauri::Result<()> {
  log::info!("打开主面板");
  match app.get_webview_window(WIN_LABEL_TRAY_PANE) {
    Some(win) => match win.is_visible() {
      Ok(true) => win.hide(),
      Ok(false) => {
        log::info!("获取当前屏幕");
        let monitor = app.monitor_from_point(pos.x, pos.y).unwrap().unwrap();
        log::info!("主面板存在且是隐藏状态");
        log::info!("设置主面板位置");
        win.set_position(calc_pane_pos(win.outer_size()?, monitor, pos))?;
        log::info!("主面板显示");
        win.show()?;
        win.set_focus()
      }
      _ => Ok(()),
    },
    None => {
      log::info!("主面板不存在");
      let win = tauri::WebviewWindowBuilder::new(
        app,
        WIN_LABEL_TRAY_PANE,
        #[cfg(debug_assertions)]
        tauri::WebviewUrl::External(
          app
            .config()
            .build
            .dev_url
            .clone()
            .unwrap()
            .join(WIN_URL_TRAY_PANE)
            .unwrap(),
        ),
        #[cfg(not(debug_assertions))]
        tauri::WebviewUrl::Url(format!("index.html{}", WIN_URL_TRAY_PANE)),
      )
      .visible(false)
      .inner_size(350., 600.)
      .decorations(false)
      .skip_taskbar(true)
      .always_on_top(true)
      .maximizable(false)
      .resizable(false)
      .build()?;
      log::info!("获取当前屏幕");
      let monitor = app.monitor_from_point(pos.x, pos.y).unwrap().unwrap();
      log::info!("设置主面板位置");
      win.set_position(calc_pane_pos(win.outer_size()?, monitor, pos))?;
      win.show()?;
      win.set_focus()?;
      Ok(())
    }
  }
}
