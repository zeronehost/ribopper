use chrono::{DateTime, Local};

use crate::models::FromRow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cmd {
  pub name: String,
  pub description: Option<String>,
  pub command: String,
  pub cwd: Option<String>,
  pub args: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
  pub id: u64,
  pub name: String,
  pub description: Option<String>,
  pub reg: String,
  pub commands: Vec<Cmd>,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for Command {
  fn from_row(row: &rusqlite::Row) -> crate::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      name: row.get(1)?,
      description: row.get(2)?,
      reg: row.get(3)?,
      commands: serde_json::from_str(&row.get::<_, String>(4)?)?,
      created_at: row.get(5)?,
      updated_at: row.get(6)?,
    })
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewCommand {
  pub name: String,
  pub description: Option<String>,
  pub reg: String,
  pub commands: Vec<Cmd>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateCommand {
  pub id: u64,
  pub name: String,
  pub description: Option<String>,
  pub reg: String,
  pub commands: Vec<Cmd>,
}