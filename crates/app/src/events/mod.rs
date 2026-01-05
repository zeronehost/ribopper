use tauri::{AppHandle, Emitter, Runtime};

use crate::utils::constant::RIBO_EVENT;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboEvent<D> {
  #[serde(rename = "type")]
  typ: EventType,
  label: EventLabel,
  payload: Option<D>,
}

impl<D> RiboEvent<D>
where
  D: serde::Serialize + Clone,
{
  fn new(typ: EventType, payload: Option<D>, label: EventLabel) -> Self {
    Self {
      typ,
      payload,
      label,
    }
  }

  #[allow(unused)]
  pub fn create_init_event(payload: Option<D>, label: EventLabel) -> Self {
    Self::new(EventType::Init, payload, label)
  }

  pub fn create_update_event(payload: Option<D>, label: EventLabel) -> Self {
    Self::new(EventType::Update, payload, label)
  }

  pub fn emit<R: Runtime>(&self, app: &AppHandle<R>) -> tauri::Result<()> {
    log::debug!(
      "events: emitting {:?} to {} (label={})",
      self.typ,
      RIBO_EVENT,
      self.label
    );
    app.emit(RIBO_EVENT, self)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
  Init,
  Update,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventLabel {
  Config,
  Record,
  Action,
  ActionOption,
  Option,
  Target,
  ALL,
}

impl core::fmt::Display for EventLabel {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      EventLabel::Config => write!(f, "config"),
      EventLabel::Record => write!(f, "record"),
      EventLabel::Action => write!(f, "action"),
      EventLabel::ActionOption => write!(f, "action-option"),
      EventLabel::Option => write!(f, "option"),
      EventLabel::Target => write!(f, "target"),
      EventLabel::ALL => write!(f, "all"),
    }
  }
}