use crate::utils::{
  constant::{WIN_LABEL_MAIN, WIN_LABEL_TRAY_PANE, WIN_NANE, WIN_URL_SETTING, WIN_URL_TRAY_PANE},
  pos::set_tray_window_pos,
};
use tauri::{AppHandle, Manager, Runtime};

pub fn open_setting_window<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
  match app.get_webview_window(WIN_LABEL_MAIN) {
    Some(win) => match win.is_visible() {
      Ok(true) => win.hide(),
      Ok(false) => {
        win.show()?;
        win.set_focus()
      }
      _ => Ok(()),
    },
    None => {
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
        tauri::WebviewUrl::App(format!("index.html{}", WIN_URL_SETTING).into()),
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
) -> tauri::Result<()> {
  match app.get_webview_window(WIN_LABEL_TRAY_PANE) {
    Some(win) => match win.is_visible() {
      Ok(true) => win.hide(),
      Ok(false) => {
        set_tray_window_pos(app, &win)?;
        win.show()?;
        win.set_focus()
      }
      _ => Ok(()),
    },
    None => {
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
        tauri::WebviewUrl::App(format!("index.html{}", WIN_URL_TRAY_PANE).into()),
      )
      .visible(false)
      .inner_size(350., 600.)
      .decorations(false)
      .skip_taskbar(!cfg!(debug_assertions))
      .always_on_top(!cfg!(debug_assertions))
      .maximizable(false)
      .resizable(false)
      .devtools(cfg!(debug_assertions))
      .build()?;
      
      set_tray_window_pos(app, &win)?;
      win.show()?;
      if cfg!(debug_assertions) {
        win.open_devtools();
      }
      win.set_focus()?;
      Ok(())
    }
  }
}
