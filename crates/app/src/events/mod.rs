use tauri::{AppHandle, Emitter, Runtime};

use crate::utils::{constant::RIBO_EVENT, error::Result};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RiboEvent {
  #[serde(rename = "type")]
  typ: EventType,
  label: EventLabel,
  action: EventAction,
}

impl RiboEvent
{
  fn new(typ: EventType, label: EventLabel, action: EventAction) -> Self {
    Self {
      typ,
      label,
      action,
    }
  }

  #[allow(unused)]
  pub(crate) fn create_init_event(label: EventLabel, action: EventAction) -> Self {
    Self::new(EventType::Init, label, action)
  }

  pub(crate) fn create_update_event(label: EventLabel, action: EventAction) -> Self {
    Self::new(EventType::Update, label, action)
  }

  pub(crate) fn emit<R: Runtime>(&self, app: &AppHandle<R>) -> Result<()> {
    log::debug!(
      "events: emitting {:?} to {} (label={}, action={})",
      self.typ,
      RIBO_EVENT,
      self.label,
      self.action
    );
    app.emit(RIBO_EVENT, self).map_err(Into::into)
  }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
  Init,
  Update,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventLabel {
  Config,
  Record,
  #[cfg(feature = "action")]
  Action,
  #[cfg(feature = "action")]
  ActionOption,
  #[cfg(feature = "action")]
  Option,
  Target,
}

impl core::fmt::Display for EventLabel {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      EventLabel::Config => write!(f, "config"),
      EventLabel::Record => write!(f, "record"),
      #[cfg(feature = "action")]
      EventLabel::Action => write!(f, "action"),
      #[cfg(feature = "action")]
      EventLabel::ActionOption => write!(f, "action-option"),
      #[cfg(feature = "action")]
      EventLabel::Option => write!(f, "option"),
      EventLabel::Target => write!(f, "target"),
    }
  }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventAction {
  Create,
  Update,
  Read,
  Delete,
  Clear,
  Save,
}

impl core::fmt::Display for EventAction {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      EventAction::Create => write!(f, "create"),
      EventAction::Update => write!(f, "update"),
      EventAction::Read => write!(f, "read"),
      EventAction::Delete => write!(f, "delete"),
      EventAction::Clear => write!(f, "clear"),
      EventAction::Save => write!(f, "save"),
    }
  }
}