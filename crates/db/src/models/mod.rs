use chrono::{DateTime, Local};
use rusqlite::{
  Result, ToSql,
  types::{FromSql, ToSqlOutput, Value},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
  pub id: usize,
  pub content: String,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
  #[serde(rename = "type")]
  pub typ: HistoryType,
  pub favorites: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewHistory {
  pub content: String,
  #[serde(rename = "type")]
  pub typ: HistoryType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateHistory {
  pub id: usize,
  pub content: String,
  #[serde(rename = "type")]
  pub typ: HistoryType,
  pub favorites: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryHistory {
  pub list: Vec<History>,
  pub total: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HistoryType {
  Text,
  Image,
  File,
  Dir,
}

impl ToSql for HistoryType {
  fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
    match self {
      Self::Text => Ok(ToSqlOutput::Owned(Value::Integer(0))),
      Self::Image => Ok(ToSqlOutput::Owned(Value::Integer(1))),
      Self::File => Ok(ToSqlOutput::Owned(Value::Integer(2))),
      Self::Dir => Ok(ToSqlOutput::Owned(Value::Integer(3))),
    }
  }
}

impl FromSql for HistoryType {
  fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
    match value {
      rusqlite::types::ValueRef::Integer(i) => match i {
        0 => Ok(Self::Text),
        1 => Ok(Self::Image),
        2 => Ok(Self::File),
        3 => Ok(Self::Dir),
        _ => Err(rusqlite::types::FromSqlError::InvalidType),
      },
      _ => Err(rusqlite::types::FromSqlError::InvalidType),
    }
  }
}
