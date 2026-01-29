use super::db::Database;
use crate::Result;
use rusqlite::params;
use tracing::instrument;

impl Database {
  #[instrument(skip_all)]
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
    tracing::debug!("current version={:?}", version);
    Ok(version)
  }

  #[instrument(skip_all)]
  pub(super) fn migration_version_exists(&self) -> Result<bool> {
    match self.get_migration_version() {
      Ok(Some(_)) => {
        tracing::debug!("version exists");
        Ok(true)
      }
      Ok(None) => {
        tracing::debug!("version does not exist");
        Ok(false)
      }
      Err(_) => {
        tracing::error!("error checking migration version");
        Ok(false)
      }
    }
  }

  #[instrument(skip(self))]
  fn update_migration_version(&self, version: u16) -> Result<()> {
    self.0.execute(
      "insert into schema_version (version) values (?1);",
      params![version],
    )?;
    tracing::debug!("updated migration version to {}", version);
    Ok(())
  }

  #[instrument(skip(self))]
  pub fn migrate_after_version(&self, version: u16) -> Result<()> {
    for migrate in crate::migration::MIGRATIONS.iter() {
      if migrate.version > version {
        tracing::info!(
          "migrating from version {} to {}",
          version,
          migrate.version
        );
        tracing::info!("{}", migrate.description);

        // 迁移数据
        let data_sql = if cfg!(feature = "migrate") {
          migrate.migrate(self.conn())?
        } else {
          "".to_string()
        };
        println!("|{}\n{}|", migrate.description, data_sql);
        match self.0.execute_batch(&format!(
          "BEGIN;\n{}\n{}\nCOMMIT;",
          migrate.script, data_sql
        )) {
          Ok(_) => {
            self.update_migration_version(migrate.version)?;
            tracing::info!("migration {} applied", migrate.version);
          }
          Err(e) => {
            tracing::error!(
              "failed to apply migration {}: {}",
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
