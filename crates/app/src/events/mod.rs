#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboEvent<D> {
  #[serde(rename = "type")]
  typ: EventType,
  payload: Option<D>,
}

impl<D> RiboEvent<D>
where
  D: serde::Serialize + Clone,
{
  fn new(typ: EventType, payload: Option<D>) -> Self {
    Self { typ, payload }
  }

  pub fn create_init_event(payload: Option<D>) -> Self {
    Self::new(EventType::Init, payload)
  }

  pub fn create_update_event(payload: Option<D>) -> Self {
    Self::new(EventType::Update, payload)
  }

  pub fn create_refresh_event(payload: Option<D>) -> Self {
    Self::new(EventType::Resfresh, payload)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
  Init,
  Update,
  Resfresh,
}
