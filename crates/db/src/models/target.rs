use super::FromRow;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
  pub id: u64,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for Target {
  fn from_row(row: &rusqlite::Row) -> crate::error::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      name: row.get(1)?,
      description: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewTarget {
  pub name: String,
  pub description: Option<String>,
}
