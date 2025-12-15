use std::path::Path;

use crate::error::Result;
use rusqlite::Connection;

pub struct Database(pub Connection);

impl Database {
  fn get_connection<P: AsRef<Path>>(path: P, key: Option<&str>) -> Result<Connection> {
    let conn = Connection::open(path)?;
    if let Some(key) = key {
      conn.pragma_update(None, "key", key)?;
    }
    Ok(conn)
  }

  pub fn new<P: AsRef<Path>>(path: P, key: Option<&str>) -> Result<Self> {
    Ok(Self(Self::get_connection(path, key)?))
  }

  pub fn init(&self) -> Result<()> {
    log::info!("Initializing database");
    let schema_version_exists = self.schema_version_exists()?;
    if !schema_version_exists {
      self.migrate_after_version(0)?;
    } else {
      let current_version = self.get_schema_version()?;
      self.migrate_after_version(current_version.unwrap_or(0))?;
    }
    Ok(())
  }

  pub fn conn(&self) -> &Connection {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_init() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    std::fs::remove_file(&uri).unwrap();
    let db = Database::new(uri, None).unwrap();
    let res = db.init();
    println!("{:?}", res);
    assert!(res.is_ok());
  }
}
