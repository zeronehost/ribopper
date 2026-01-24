use crate::{Result, migration::Migrate, models::FromRow};

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

impl Migrate for RecordAction {
  fn migrate(db: &rusqlite::Connection) -> crate::Result<String> {
    // 原数据结构
    let mut stmt =
      db.prepare("select id, record_id, action_id from record_action")?;
    let data = stmt.query_map(rusqlite::params![], |row| {
      // 新数据结构
      Ok(format!(
          "insert into record_action (id, record_id, action_id) values ({}, {}, {});\n",
          row.get::<usize, u64>(0)?,
          row.get::<usize, u64>(1)?,
          row.get::<usize, u64>(2)?,
        ))
    })?;
    let mut sql = String::new();
    for row in data {
      sql.push_str(row?.as_str());
    }
    Ok(sql)
  }
}