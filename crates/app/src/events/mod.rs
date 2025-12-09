use tauri::{AppHandle, Emitter, EventTarget, Runtime};

use crate::utils::constant::RIBO_EVENT;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboEvent<D> {
  #[serde(rename = "type")]
  typ: EventType,
  label: String,
  payload: Option<D>,
}

impl<D> RiboEvent<D>
where
  D: serde::Serialize + Clone,
{
  fn new(typ: EventType, payload: Option<D>, label: &str) -> Self {
    Self { typ, payload, label: label.to_string()}
  }

  pub fn create_init_event(payload: Option<D>, label: &str) -> Self {
    Self::new(EventType::Init, payload, label)
  }

  pub fn create_update_event(payload: Option<D>, label: &str) -> Self {
    Self::new(EventType::Update, payload, label)
  }

  pub fn create_refresh_event(payload: Option<D>, label: &str) -> Self {
    Self::new(EventType::Refresh, payload, label)
  }

  pub fn emit<R: Runtime>(&self, app: &AppHandle<R>) -> tauri::Result<()> {
    app.emit(RIBO_EVENT, self)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
  Init,
  Update,
  Refresh,
}
