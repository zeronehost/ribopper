use rusqlite::params;

use crate::{
  Database, Result,
  models::{self, FromRow},
};

impl Database {
  pub fn create_command(&self, command: &models::NewCommand) -> Result<models::Command> {
    log::info!(
      "db.command: create_command invoked (name={:?})",
      command.name
    );
    let mut stmt = self.conn().prepare("insert into command (name, description, reg, commands) values (?1, ?2, ?3, ?4) RETURNING id, name, description, reg, commands, created_at, updated_at, updated_at")?;
    stmt.query_row(
      params![
        command.name,
        command.description,
        command.reg,
        serde_json::to_string(&command.commands)?
      ],
      |row| Ok(models::Command::from_row(row)),
    )?
  }

  pub fn get_command_by_id(&self, id: u64) -> Result<Option<models::Command>> {
    log::info!("db.command: get_command_by_id id={}", id);
    let mut stmt = self.conn().prepare("select id, name, description, reg, commands, created_at, updated_at, updated_at from command where id = ?1")?;
    match stmt.query_row(params![id], |row| Ok(models::Command::from_row(row))) {
      Ok(command) => Ok(Some(command?)),
      Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub fn get_command_by_reg(&self, reg: &str) -> Result<Option<models::Command>> {
    log::info!("db.command: get_command_by_reg reg={}", reg);
    let mut stmt = self.conn().prepare("select id, name, description, reg, commands, created_at, updated_at, updated_at from command where reg = ?1")?;
    match stmt.query_row(params![reg], |row| Ok(models::Command::from_row(row))) {
      Ok(command) => Ok(Some(command?)),
      Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub fn get_commands(&self) -> Result<Vec<models::Command>> {
    log::info!("db.command: get_commands invoked");
    let mut stmt = self.conn().prepare("select id, name, description, reg, commands, created_at, updated_at, updated_at from command")?;
    let commands = stmt.query_map(params![], |row| Ok(models::Command::from_row(row)))?;
    let mut result = Vec::new();
    for command in commands {
      result.push(command??);
    }
    Ok(result)
  }

  pub fn delete_command(&self, id: u64) -> Result<bool> {
    log::info!("db.command: delete_command invoked id={}", id);
    let mut stmt = self.conn().prepare("delete from command where id = ?1")?;
    let rows_affected = stmt.execute(params![id])?;
    Ok(rows_affected > 0)
  }
}

#[cfg(test)]
mod tests {
    use crate::{Database, models};

  fn init_data(db: &Database) {
    db.conn()
      .execute_batch(
        "BEGIN TRANSACTION; DELETE FROM command; DELETE FROM sqlite_sequence WHERE name = 'command';commit;")
      .unwrap();
  }
  #[test]
  fn test_create_command() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();
    init_data(&db);
    let command = db.create_command(&models::NewCommand {
      name: "test".to_string(),
      description: Some("test".to_string()),
      reg: "*".to_string(),
      commands: vec![],
    });
    println!("{:?}", command);
    assert!(command.is_ok());
  }

  #[test]
  fn test_get_command_by_id() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();

    let command = db.get_command_by_id(1);
    println!("{:?}", command);
    assert!(command.is_ok());
  }

  #[test]
  fn test_get_command_by_reg() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();

    let command = db.get_command_by_reg("*");
    println!("{:?}", command);
    assert!(command.is_ok());
  }

  #[test]
  fn test_get_commands() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();

    let command = db.get_commands();
    println!("{:?}", command);
    assert!(command.is_ok());
  }

  #[test]
  fn test_delete_command() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();

    let command = db.delete_command(1);
    println!("{:?}", command);
    assert!(command.is_ok());
  }
}