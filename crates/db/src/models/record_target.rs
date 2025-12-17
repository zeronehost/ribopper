use super::FromRow;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordWithTargets {
  pub record_id: u64,
  pub content: String,
  pub data: String,
  pub record_created: Option<DateTime<Local>>,
  pub record_updated: Option<DateTime<Local>>,
  pub target_names: Vec<String>,
  pub target_count: u64,
}

impl FromRow for RecordWithTargets {
  fn from_row(row: &rusqlite::Row) -> crate::error::Result<Self> {
    let value: String = row.get(5)?;
    let target_names: Vec<String> = if value.is_empty() || value == "null" || value == "[null]" {
      vec![]
    } else {
      serde_json::from_str(&value)?
    };
    Ok(Self {
      record_id: row.get(0)?,
      content: row.get(1)?,
      data: row.get(2)?,
      record_created: row.get(3)?,
      record_updated: row.get(4)?,
      target_names,
      target_count: row.get(6)?,
    })
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordTarget {
  pub id: u64,
  pub target_id: u64,
  pub record_id: u64,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for RecordTarget {
  fn from_row(row: &rusqlite::Row) -> crate::error::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      target_id: row.get(1)?,
      record_id: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewRecordTarget {
  pub target_id: u64,
  pub record_id: u64,
}
