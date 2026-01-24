use chrono::{DateTime, Local};
use rusqlite::{ToSql, types::FromSql};

use crate::{migration::Migrate, models::FromRow};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
  pub id: u64,
  pub name: String,
  pub description: Option<String>,
  pub pattern: String,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for Action {
  fn from_row(row: &rusqlite::Row) -> crate::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      name: row.get(1)?,
      description: row.get(2)?,
      pattern: row.get(3)?,
      created_at: row.get(4)?,
      updated_at: row.get(5)?,
    })
  }
}

impl Migrate for Action {
  fn migrate(db: &rusqlite::Connection) -> crate::Result<String> {
    // 原数据结构
    let mut stmt =
      db.prepare("select id, name, description, pattern, created_at, updated_at from action")?;
    let data = stmt.query_map(rusqlite::params![], |row| {
      // 新数据结构
      Ok(format!(
          "insert into action (id, name, description, pattern, created_at, updated_at) values ({}, '{}', '{}', '{}', '{}', '{}');\n",
          row.get::<usize, u64>(0)?,
          row.get::<usize, String>(1)?,
          row.get::<usize, String>(2)?,
          row.get::<usize, String>(3)?,
          row.get::<usize, String>(4)?,
          row.get::<usize, String>(5)?
        ))
    })?;
    let mut sql = String::new();
    for row in data {
      sql.push_str(row?.as_str());
    }
    Ok(sql)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewAction {
  pub description: Option<String>,
  pub pattern: String,
  pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateAction {
  pub id: u64,
  pub description: Option<String>,
  pub pattern: String,
  pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiboOption {
  pub id: u64,
  pub description: Option<String>,
  pub command: String,
  pub name: String,
  pub action_id: u64, // foreign key
  pub out: OutModel,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for RiboOption {
  fn from_row(row: &rusqlite::Row) -> crate::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      action_id: row.get(1)?,
      name: row.get(2)?,
      description: row.get(3)?,
      command: row.get(4)?,
      out: row.get(5)?,
      created_at: row.get(6)?,
      updated_at: row.get(7)?,
    })
  }
}

impl Migrate for RiboOption {
  fn migrate(db: &rusqlite::Connection) -> crate::Result<String> {
    // 原数据结构
    let mut stmt = db.prepare(
      "select id, action_id, name, description, command, out, created_at, updated_at from action",
    )?;
    let data = stmt.query_map(rusqlite::params![], |row| {
      // 新数据结构
      Ok(format!(
          "insert into action (id, action_id, name, description, command, out, created_at, updated_at) values ({}, '{}', '{}', '{}', '{}', '{}', '{}', '{}');\n",
          row.get::<usize, u64>(0)?,
          row.get::<usize, String>(1)?,
          row.get::<usize, String>(2)?,
          row.get::<usize, String>(3)?,
          row.get::<usize, String>(4)?,
          row.get::<usize, String>(5)?,
          row.get::<usize, String>(6)?,
          row.get::<usize, String>(7)?,
        ))
    })?;
    let mut sql = String::new();
    for row in data {
      sql.push_str(row?.as_str());
    }
    Ok(sql)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRiboOption {
  pub description: Option<String>,
  pub action_id: u64,
  pub command: String,
  pub name: String,
  pub out: OutModel,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateRiboOption {
  pub id: u64,
  pub description: Option<String>,
  pub command: String,
  pub name: String,
  pub out: OutModel,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionWithOption {
  pub id: u64,
  pub name: String,
  pub description: Option<String>,
  pub pattern: String,
  pub options: Vec<RiboOption>,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl From<Action> for ActionWithOption {
  fn from(val: Action) -> Self {
    ActionWithOption {
      id: val.id,
      name: val.name,
      description: val.description,
      pattern: val.pattern,
      options: vec![],
      created_at: val.created_at,
      updated_at: val.updated_at,
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewActionWithOption {
  pub description: Option<String>,
  pub pattern: String,
  pub name: String,
  pub options: Vec<NewRiboOption>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutModel {
  Ingore,
  Replace,
  Append,
}

impl FromSql for OutModel {
  fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
    match value.as_str()? {
      "i" => Ok(OutModel::Ingore),
      "r" => Ok(OutModel::Replace),
      "a" => Ok(OutModel::Append),
      _ => Ok(OutModel::Ingore),
    }
  }
}

impl ToSql for OutModel {
  fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
    Ok(match self {
      OutModel::Ingore => "i".into(),
      OutModel::Replace => "r".into(),
      OutModel::Append => "a".into(),
    })
  }
}
