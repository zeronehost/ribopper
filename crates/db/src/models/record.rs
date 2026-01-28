use crate::migration::Migrate;

use super::FromRow;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
  pub id: u64,
  pub content: String,
  #[serde(rename = "type")]
  pub typ: RecordType,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for Record {
  fn from_row(row: &rusqlite::Row) -> crate::error::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      content: row.get(1)?,
      typ: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  }
}

impl Migrate for Record {
  fn migrate(_db: &rusqlite::Connection) -> crate::Result<&'static str> {
    // 原数据结构
    Ok("alter table record drop column data;")
  }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordType {
  Text,
  #[cfg(feature = "image")]
  Image,
  #[cfg(feature = "file")]
  Files,
}

impl rusqlite::types::FromSql for RecordType {
  fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
    match value.as_str()? {
      "text" => Ok(RecordType::Text),
      #[cfg(feature = "image")]
      "image" => Ok(RecordType::Image),
      #[cfg(feature = "file")]
      "files" => Ok(RecordType::Files),
      _ => Err(rusqlite::types::FromSqlError::InvalidType),
    }
  }
}

impl rusqlite::types::ToSql for RecordType {
  fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
    Ok(rusqlite::types::ToSqlOutput::Owned(
      match self {
        RecordType::Text => "text".to_string(),
        #[cfg(feature = "image")]
        RecordType::Image => "image".to_string(),
        #[cfg(feature = "file")]
        RecordType::Files => "files".to_string(),
      }
      .into(),
    ))
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewRecord {
  pub content: String,
  #[serde(rename = "type")]
  pub typ: RecordType,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordQuery {
  pub content_contains: Option<String>,
  // pub target_id: Option<i64>,
  pub start_date: Option<DateTime<Local>>,
  pub end_date: Option<DateTime<Local>>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
  pub order_by: Option<String>,
  pub order_direction: Option<String>,
}
