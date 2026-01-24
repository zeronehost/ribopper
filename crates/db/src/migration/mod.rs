use std::sync::LazyLock;

#[cfg(feature = "action")]
use crate::models::{Action, RecordAction};
use crate::{
  Result,
  models::Record,
};

pub const SCHEMA: &str = include_str!("../../sql/init.sql");
pub const RECOED_SCHEMA: &str = include_str!("../../sql/record.sql");

pub const ACTION_SCHEMA: &str = if cfg!(feature = "action") {
  include_str!("../../sql/action.sql")
} else {
  ""
};

pub(crate) trait Migrate {
  fn migrate(db: &rusqlite::Connection) -> Result<String>;
}

pub struct Migration {
  pub version: u16,
  pub script: String,
  pub description: String,
}

impl Migration {
  pub fn new(version: u16, script: &str, description: &str) -> Self {
    Self {
      version,
      script: script.to_string(),
      description: description.to_string(),
    }
  }

  pub fn migrate(&self, db: &rusqlite::Connection) -> Result<String> {
    let mut stmt = db.prepare("select name from sqlite_master where type='table';")?;
    let mut rows = stmt.query([])?;
    let mut sql = String::new();
    while let Some(row) = rows.next()? {
      let name: String = row.get(0)?;
      match name.as_str() {
        "record" => {
          sql.push_str(Record::migrate(db)?.as_str());
        }
        #[cfg(feature = "action")]
        "actions" => {
          sql.push_str(Action::migrate(db)?.as_str());
        }
        #[cfg(feature = "action")]
        "record_action" => {
          sql.push_str(RecordAction::migrate(db)?.as_str());
        }
        #[cfg(feature = "action")]
        "options" => {
          sql.push_str(RiboOptions::migrate(db)?.as_str());
        }
        _ => {
          break;
        }
      }
    }
    Ok(sql)
  }
}

pub static MIGRATIONS: LazyLock<Vec<Migration>> = LazyLock::new(|| {
  vec![Migration::new(
    1,
    &format!("{}{}{}", SCHEMA, RECOED_SCHEMA, ACTION_SCHEMA),
    "Initial schema",
  )]
});
