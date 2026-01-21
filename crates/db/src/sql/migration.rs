use super::db::Database;
use crate::Result;
use rusqlite::params;

impl Database {
  pub(super) fn get_migration_version(&self) -> Result<Option<u16>> {
    let mut stmt = self
      .0
      .prepare("select version from schema_version order by version desc limit 1;")?;
    let rows = stmt.query_map(params![], |row| row.get::<usize, u16>(0))?;

    let mut versions = Vec::new();
    for version in rows {
      versions.push(version?);
    }
    let version = versions.first().copied();
    log::debug!("db.migration: current version={:?}", version);
    Ok(version)
  }

  pub(super) fn migration_version_exists(&self) -> Result<bool> {
    match self.get_migration_version() {
      Ok(Some(_)) => {
        log::debug!("db.migration: version exists");
        Ok(true)
      }
      Ok(None) => {
        log::debug!("db.migration: version does not exist");
        Ok(false)
      }
      Err(_) => {
        log::error!("db.migration: error checking migration version");
        Ok(false)
      }
    }
  }

  fn update_migration_version(&self, version: u16) -> Result<()> {
    self.0.execute(
      "insert into schema_version (version) values (?1);",
      params![version],
    )?;
    log::debug!("db.migration: updated migration version to {}", version);
    Ok(())
  }
  pub fn migrate_after_version(&self, version: u16) -> Result<()> {
    for migrate in crate::migration::MIGRATIONS.iter() {
      if migrate.version > version {
        log::info!(
          "db.migration: migrating from version {} to {}",
          version,
          migrate.version
        );
        log::info!("db.migration: {}", migrate.description);

        // 迁移数据
        let data_sql = if cfg!(feature = "migrate") {
          migrate.migrate(self.conn())?
        } else {
          "".to_string()
        };

        match self
          .0
          .execute_batch(&format!("BEGIN; {}\n{} COMMIT;", migrate.script, data_sql))
        {
          Ok(_) => {
            self.update_migration_version(migrate.version)?;
            log::info!("db.migration: migration {} applied", migrate.version);
          }
          Err(e) => {
            log::error!(
              "db.migration: failed to apply migration {}: {}",
              migrate.version,
              e
            );
            return Err(e.into());
          }
        }
      }
    }
    Ok(())
  }
}
