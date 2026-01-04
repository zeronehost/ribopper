use chrono::{DateTime, Local};

use crate::models::FromRow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
  pub id: u64,
  pub description: Option<String>,
  pub pattern: String,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for Action {
  fn from_row(row: &rusqlite::Row) -> crate::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      description: row.get(1)?,
      pattern: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewAction {
  pub description: Option<String>,
  pub pattern: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateAction {
  pub id: u64,
  pub description: Option<String>,
  pub pattern: String,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiboOption {
  pub id: u64,
  pub description: Option<String>,
  pub command: String,
  pub action_id: u64, // foreign key
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl FromRow for RiboOption {
  fn from_row(row: &rusqlite::Row) -> crate::Result<Self> {
    Ok(Self {
      id: row.get(0)?,
      action_id: row.get(1)?,
      description: row.get(2)?,
      command: row.get(3)?,
      created_at: row.get(4)?,
      updated_at: row.get(5)?,
    })
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRiboOption {
  pub description: Option<String>,
  pub action_id: u64,
  pub command: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateRiboOption {
  pub id: u64,
  pub description: Option<String>,
  pub command: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionWithOption {
  pub id: u64,
  pub description: Option<String>,
  pub pattern: String,
  pub options: Vec<RiboOption>,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}

impl Into<ActionWithOption> for Action {
  fn into(self) -> ActionWithOption {
    ActionWithOption {
      id: self.id,
      description: self.description,
      pattern: self.pattern,
      options: vec![],
      created_at: self.created_at,
      updated_at: self.updated_at,
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewActionWithOption {
  pub description: Option<String>,
  pub pattern: String,
  pub options: Vec<NewRiboOption>,
}