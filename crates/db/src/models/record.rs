use super::FromRow;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
  pub id: u64,
  pub content: String,
  pub data: String,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for Record {
  fn from_row(row: &rusqlite::Row) -> crate::error::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      content: row.get(1)?,
      data: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewRecord {
  pub content: String,
  pub data: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordQuery {
  pub content_contains: Option<String>,
  pub target_id: Option<i64>,
  pub start_date: Option<DateTime<Local>>,
  pub end_date: Option<DateTime<Local>>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
  pub order_by: Option<String>,
  pub order_direction: Option<String>,
}
