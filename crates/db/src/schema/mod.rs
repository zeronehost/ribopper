use std::sync::LazyLock;

pub const SCHEMA: &str = include_str!("../../sql/init.sql");
pub const RECOED_SCHEMA: &str = include_str!("../../sql/record.sql");

pub const ACTION_SCHEMA: &str = if cfg!(feature = "action") { include_str!("../../sql/action.sql") } else { "" };

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
}

pub static MIGRATIONS: LazyLock<Vec<Migration>> =
  LazyLock::new(|| vec![Migration::new(1, &format!("{}{}{}", SCHEMA, RECOED_SCHEMA, ACTION_SCHEMA), "Initial schema")]);
