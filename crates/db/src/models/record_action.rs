use crate::{Result, models::FromRow};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecordAction {
  pub id: u64,
  pub record_id: u64,
  pub action_id: u64,
}

impl FromRow for RecordAction {
  fn from_row(row: &rusqlite::Row) -> Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      record_id: row.get(1)?,
      action_id: row.get(2)?,
    })
  }
}