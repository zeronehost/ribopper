use std::{path::Path};

use rusqlite::{Connection, Result, params};

use crate::models::{History, NewHistory, UpdateHistory};

pub mod models;
mod schema;

pub struct Database(pub Connection);

impl Database {
  fn get_connection<P: AsRef<Path>>(p: P, key: Option<String>) -> Result<Connection> {
    let connection = Connection::open(p)?;
    if let Some(key) = key {
      connection.pragma_update(None, "key", &key)?;
    }
    Ok(connection)
  }

  pub fn new<P: AsRef<Path>>(p: P, key: Option<String>) -> Result<Self> {
    let connection = Self::get_connection(p, key)?;
    Ok(Self(connection))
  }

  pub fn init(&self) -> Result<()> {
    let schema_version_exists = self.schema_version_exists()?;
    if !schema_version_exists {
      self.migrate_after_version(0)?;
    } else {
      let current_version = self.get_schema_version()?;
      self.migrate_after_version(current_version.unwrap())?;
    }
    Ok(())
  }
}

impl Database {
  fn get_schema_version(&self) -> Result<Option<u16>> {
    let mut stmt = self
      .0
      .prepare("select version for ribo_schema order by version desc limit 1;")?;
    let version_iter = stmt.query_map(params![], |row| Ok(row.get::<usize, u16>(0)?))?;

    let mut versions = Vec::new();
    for version in version_iter {
      versions.push(version?);
    }

    Ok(versions.first().copied())
  }

  fn schema_version_exists(&self) -> Result<bool> {
    match self.get_schema_version() {
      Ok(Some(_)) => Ok(true),
      _ => Ok(false),
    }
  }

  fn update_schema_version(&self, version: u16) -> Result<()> {
    self.0.execute(
      "insert into ribo_schema (version) values (?1);",
      params![version],
    )?;
    Ok(())
  }

  fn migrate_after_version(&self, version: u16) -> Result<()> {
    for migration in crate::schema::MIGRATIONS.iter() {
      if migration.version > version {
        log::info!(
          "migrating from version {} to {}",
          version,
          migration.version
        );
        log::info!("{}", migration.description);

        match self
          .0
          .execute_batch(&format!("BEGIN; {} COMMIT;", migration.script))
        {
          Ok(_) => self.update_schema_version(migration.version)?,
          Err(e) => {
            log::error!("failed to apply migration: {}", e);
            return Err(e);
          }
        }
      }
    }
    Ok(())
  }
}

impl Database {
  pub fn create_data(&self, data: NewHistory) -> Result<()> {
    self.0.execute(
      "insert into history (content, type) values (?1, ?2);",
      params![data.content, data.typ],
    )?;
    Ok(())
  }

  pub fn update_data(&self, data: UpdateHistory) -> Result<()> {
    self.0.execute(
      "update history set content = ?1 where id = ?2;",
      params![data.content, data.id],
    )?;
    Ok(())
  }

  pub fn delete_data(&self, id: usize) -> Result<()> {
    self
      .0
      .execute("delete from history where id = ?1;", params![id])?;
    Ok(())
  }

  pub fn query_data(&self, index: usize, size: usize) -> Result<Vec<History>> {
    let mut stmt = self.0.prepare("select * from history limit ?1 offset ?2")?;
    let list_iter = stmt.query_map(params![size, index * size], |row| Ok(History {
      id: row.get(0)?,
      content: row.get(1)?,
      typ: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    }))?;

    let mut list = vec![];
    for item in list_iter {
      list.push(item?);
    }

    Ok(list)
  }

  pub fn query_total(&self) -> Result<usize> {
    let mut stmt = self.0.prepare("select count(*) from history;")?;
    stmt.query_one(params![], |row| Ok(row.get::<usize, usize>(0)?))    
  }

  pub fn clear_data(&self) -> Result<()> {
    match self.0.execute_batch("BEGIN TRANSACTION; DELETE FROM history; DELETE FROM sqlite_sequence WHERE name = 'history'; COMMIT;") {
      Ok(_) => Ok(()),
      Err(e) => Err(e),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::models::HistoryType;

  use super::*;

  fn init() -> anyhow::Result<Database> {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();

    Ok(db)
  }

  #[test]
  fn test_connection_db() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();
    let inited = db.init();

    assert!(inited.is_ok());
  }

  #[test]
  fn test_create_data() {
    let db = init().unwrap();

    let data = NewHistory {
      content: "test".to_string(),
      typ: HistoryType::Text,
    };

    let res = db.create_data(data);

    println!("{:?}", res);

    assert!(res.is_ok());
  }

  #[test]
  fn test_update_data() {
    let db = init().unwrap();

    let data = UpdateHistory {
      id: 1,
      content: "test1".to_string(),
      typ: HistoryType::Text,
    };

    let res = db.update_data(data);

    println!("{:?}", res);
  }

  #[test]
  fn test_delete_data() {
    let db = init().unwrap();

    let res = db.delete_data(1);
    println!("{:?}", res);
    assert!(res.is_ok());
  }

  #[test]
  fn test_query_data() {
    let db = init().unwrap();

    let res = db.query_data(1, 1);
    println!("{:?}", res);
    assert!(res.is_ok());
  }

  #[test]
  fn test_query_total() {
    let db = init().unwrap();

    let res = db.query_total();
    println!("{:?}", res);
    assert!(res.is_ok());
  }

  #[test]
  fn test_clear_data() {
    let db = init().unwrap();

    let res = db.clear_data();
    println!("{:?}", res);
    assert!(res.is_ok());
  }
}
