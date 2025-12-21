use tauri::{AppHandle, Manager, Runtime};

use crate::{commands::config::config_load, utils::constant::WIN_LABEL_TRAY_PANE};
pub struct Clipboard(
  pub(crate) ribo_clipboard::Manager<Box<dyn Fn(ribo_clipboard::Content) + Send + 'static>>,
);

impl Clipboard {
  pub fn new<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<Self> {
    let app_handle = app.clone();
    let callback: Box<dyn Fn(ribo_clipboard::Content) + Send + 'static> =
      Box::new(move |c: ribo_clipboard::Content| {
        log::debug!("clipboard content: {:?}", c);
        let db = app_handle.state::<crate::store::db::Db>();
        let max = if let Ok(Some(config)) = config_load(app_handle.clone()) {
          config.get_max().unwrap_or(None)
        } else {
          None
        };
        match db
          .0
          .lock()
          .unwrap()
          .create_record(c.try_into().unwrap(), max)
        {
          Ok(_) => {
            crate::events::RiboEvent::<()>::create_update_event(None, WIN_LABEL_TRAY_PANE)
              .emit(&app_handle)
              .unwrap();
          }
          Err(e) => log::error!("failed to create record: {}", e),
        }
      });

    Ok(Self(ribo_clipboard::Manager::new(callback)?))
  }
}
