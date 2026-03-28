mod error;
mod tracing;

use tauri::{
  Manager, Runtime,
  plugin::{Builder, TauriPlugin},
};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::tracing::Tracing;

pub fn init<R: Runtime>(level: String) -> TauriPlugin<R> {
  Builder::new("tracing")
    // .invoke_handler()
    .setup(|app, _| {
      let env_filter = EnvFilter::try_new(level)
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("ort=warn".parse().unwrap());

      let tracing = Tracing::new(app.clone());

      let log_dir = match tracing.dirs() {
        Ok(dir) => dir,
        Err(err) => {
          eprintln!("Failed to get log directory: {}", err);
          return Ok(());
        }
      };

      if cfg!(debug_assertions) {
        tracing_subscriber::Registry::default()
          .with(env_filter)
          .with(fmt::layer())
          .init();
      } else {
        let file_appender = tracing_appender::rolling::daily(log_dir, "log");
        let (writer, guard) = tracing_appender::non_blocking(file_appender);
        tracing_subscriber::Registry::default()
          .with(env_filter)
          .with(fmt::layer())
          .with(fmt::layer().with_ansi(false).with_writer(writer))
          .init();
        app.manage(guard);
      }
      Ok(())
    })
    .build()
}
