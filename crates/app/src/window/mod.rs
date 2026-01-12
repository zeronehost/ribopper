use std::sync::{
  Arc,
  atomic::{AtomicBool, Ordering},
};

#[cfg(debug_assertions)]
use crate::utils::constant::WIN_URL_CONTEXT_PANE;
use crate::utils::{
  constant::{
    WIN_LABEL_CONTEXT_PANE, WIN_LABEL_MAIN, WIN_LABEL_TRAY_PANE, WIN_NANE, WIN_URL_SETTING,
    WIN_URL_TRAY_PANE,
  },
  pos::{set_context_window_pos, set_tray_window_pos},
};
use tauri::{AppHandle, Manager, Runtime, WindowEvent};

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

pub fn open_tray_pane<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
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
      #[cfg(debug_assertions)]
      win.open_devtools();
      win.set_focus()?;
      Ok(())
    }
  }
}

pub fn open_context_pane<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
  match app.get_webview_window(WIN_LABEL_CONTEXT_PANE) {
    Some(win) => match win.is_visible() {
      Ok(true) => win.hide(),
      Ok(false) => {
        set_context_window_pos(app, &win)?;
        win.show()?;
        win.set_focus()
      }
      _ => Ok(()),
    },
    None => {
      let win = tauri::WebviewWindowBuilder::new(
        app,
        WIN_LABEL_CONTEXT_PANE,
        #[cfg(debug_assertions)]
        tauri::WebviewUrl::External(
          app
            .config()
            .build
            .dev_url
            .clone()
            .unwrap()
            .join(WIN_URL_CONTEXT_PANE)
            .unwrap(),
        ),
        #[cfg(not(debug_assertions))]
        tauri::WebviewUrl::App(format!("index.html{}", WIN_URL_CONTEXT_PANE).into()),
      )
      .visible(false)
      .inner_size(330., 500.)
      .decorations(false)
      .skip_taskbar(!cfg!(debug_assertions))
      .always_on_top(true)
      .maximizable(false)
      .resizable(false)
      // .devtools(cfg!(debug_assertions))
      .build()?;

      set_context_window_pos(app, &win)?;

      win.show()?;

      win.set_focus()?;

      // #[cfg(debug_assertions)]
      // win.open_devtools();
      let app = app.clone();
      let is_initialized = Arc::new(AtomicBool::new(false));
      let is_initialized_clone = is_initialized.clone();

      win.on_window_event(move |ev| match ev {
        WindowEvent::Focused(focused) => {
          if !focused {
            if !is_initialized_clone.load(Ordering::SeqCst) {
              log::info!("Ignoring initial focus loss");
              return;
            }
            log::info!("Context pane window lost focus, hiding it...");
            if let Some(w) = app.get_webview_window(WIN_LABEL_CONTEXT_PANE) {
              log::info!("Context pane window found, hiding it...");
              if w.is_visible().is_ok_and(|x| x) {
                log::info!("Context pane window is visible, hiding it...");
                match w.hide() {
                  Ok(_) => log::info!("Context pane window hidden"),
                  Err(e) => log::error!("Failed to hide context pane window: {}", e),
                }
              }
            }
          } else {
            is_initialized_clone.store(true, Ordering::SeqCst);
          }
        }
        _ => {}
      });
      let is_initialized_timer = is_initialized.clone();
      std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(200));
        is_initialized_timer.store(true, Ordering::SeqCst);
      });
      Ok(())
    }
  }
}
