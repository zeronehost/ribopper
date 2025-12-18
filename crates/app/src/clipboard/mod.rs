use tauri::{AppHandle, Manager, Runtime};
pub struct Clipboard<R: Runtime> {
  app: AppHandle<R>,
}

impl<R: Runtime> Clipboard<R> {
  pub fn new(app: AppHandle<R>) -> Self {
    Self { app }
  }

  pub fn init(&self) -> anyhow::Result<()> {
    let app_handle = self.app.clone();
    let cm = ribo_clipboard::Manager::new(move |c: ribo_clipboard::Content| {
      log::info!("clipboard content: {:?}", c);
      let db = app_handle.state::<crate::store::db::Db>();
      db.0
        .lock()
        .unwrap()
        .create_record(c.try_into().unwrap())
        .unwrap();
    })?;
    self.app.manage(cm);
    Ok(())
  }
}
