use std::sync::LazyLock;

pub const SCHEMA: &str = include_str!("../../sql/init.sql");

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
  LazyLock::new(|| vec![Migration::new(1, SCHEMA, "Initial schema")]);
