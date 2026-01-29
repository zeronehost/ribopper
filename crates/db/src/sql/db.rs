use std::path::Path;

use crate::error::Result;
use rusqlite::Connection;
use tracing::instrument;

pub struct Database(pub Connection);

impl Database {
  #[instrument(skip_all)]
  fn get_connection<P: AsRef<Path>>(path: P, key: Option<&str>) -> Result<Connection> {
    tracing::info!("opening connection to path");
    let conn = Connection::open(path)?;
    if let Some(key) = key {
      tracing::debug!("applying key to connection (hidden)");
      conn.pragma_update(None, "key", key)?;
    }
    Ok(conn)
  }

  #[instrument(skip_all)]
  pub fn new<P: AsRef<Path>>(path: P, key: Option<&str>) -> Result<Self> {
    tracing::info!("creating Database at path");
    Ok(Self(Self::get_connection(path, key)?))
  }

  #[instrument(skip_all)]
  pub fn init(&self) -> Result<()> {
    tracing::info!("Initializing database");
    let version_exists = self.migration_version_exists()?;
    if !version_exists {
      self.migrate_after_version(0)?;
    } else {
      let current_version = self.get_migration_version()?;
      self.migrate_after_version(current_version.unwrap_or(0))?;
    }
    // 释放空闲占用
    let _ = self.0.execute("VACUUM;", []);
    Ok(())
  }

  #[instrument(skip_all)]
  pub fn conn(&self) -> &Connection {
    &self.0
  }

  #[instrument(skip_all)]
  pub fn conn_mut(&mut self) -> &mut Connection {
    &mut self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_init() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    if uri.exists() {
      std::fs::remove_file(&uri).unwrap();
    }
    let db = Database::new(uri, None).unwrap();
    let res = db.init();
    println!("{:?}", res);
    assert!(res.is_ok());
  }
}
