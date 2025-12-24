use super::db::Database;
use rusqlite::{Result, params};

impl Database {
  pub(super) fn get_schema_version(&self) -> Result<Option<u16>> {
    let mut stmt = self
      .0
      .prepare("select version from schema_version order by version desc limit 1;")?;
    let rows = stmt.query_map(params![], |row| row.get::<usize, u16>(0))?;

    let mut versions = Vec::new();
    for version in rows {
      versions.push(version?);
    }
    let version = versions.first().copied();
    log::debug!("db.schema: current version={:?}", version);
    Ok(version)
  }

  pub(super) fn schema_version_exists(&self) -> Result<bool> {
    match self.get_schema_version() {
      Ok(Some(_)) => {
        log::debug!("db.schema: version exists");
        Ok(true)
      }
      Ok(None) => {
        log::debug!("db.schema: version does not exist");
        Ok(false)
      }
      Err(_) => {
        log::error!("db.schema: error checking schema version");
        Ok(false)
      }
    }
  }

  fn update_schema_version(&self, version: u16) -> Result<()> {
    self.0.execute(
      "insert into schema_version (version) values (?1);",
      params![version],
    )?;
    log::debug!("db.schema: updated schema version to {}", version);
    Ok(())
  }
  pub fn migrate_after_version(&self, version: u16) -> Result<()> {
    for migrate in crate::schema::MIGRATIONS.iter() {
      if migrate.version > version {
        log::info!(
          "db.schema: migrating from version {} to {}",
          version,
          migrate.version
        );
        log::info!("db.schema: {}", migrate.description);

        match self
          .0
          .execute_batch(&format!("BEGIN; {} COMMIT;", migrate.script))
        {
          Ok(_) => {
            self.update_schema_version(migrate.version)?;
            log::info!("db.schema: migration {} applied", migrate.version);
          }
          Err(e) => {
            log::error!(
              "db.schema: failed to apply migration {}: {}",
              migrate.version,
              e
            );
            return Err(e);
          }
        }
      }
    }
    Ok(())
  }
}
