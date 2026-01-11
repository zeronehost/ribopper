use rusqlite::params;

use crate::{
  Database, Result,
  models::{self, FromRow},
};

impl Database {
  pub fn create_action(&self, new_action: models::NewAction) -> Result<models::Action> {
    log::info!(
      "db.actions: create_action invoked (pattern={:?})",
      new_action.pattern
    );
    let mut stmt = self.conn().prepare("insert into actions (description, pattern, name) values (?1, ?2, ?3) RETURNING id, name, description, pattern, created_at, updated_at")?;
    let action = stmt.query_row(
      params![new_action.description, new_action.pattern, new_action.name],
      |row| Ok(models::Action::from_row(row)),
    )??;

    Ok(action)
  }
  pub fn create_action_option(
    &self,
    new_action: models::NewActionWithOption,
  ) -> Result<models::ActionWithOption> {
    log::info!(
      "db.actions: create_action invoked (pattern={:?})",
      new_action.pattern
    );
    let mut stmt = self.conn().prepare("insert into actions (description, pattern, name) values (?1, ?2, ?3) RETURNING id, name, description, pattern, created_at, updated_at")?;
    let action = stmt
      .query_row(
        params![new_action.description, new_action.pattern, new_action.name],
        |row| Ok(models::Action::from_row(row)),
      )??
      .into();

    if new_action.options.is_empty() {
      return Ok(action);
    }
    let mut options = vec![];
    for option in new_action.options.iter() {
      let opt = self.create_option(models::NewRiboOption {
        action_id: action.id,
        ..option.clone()
      })?;
      options.push(opt);
    }

    Ok(models::ActionWithOption { options, ..action })
  }

  pub fn get_action_by_id(&self, id: u64) -> Result<Option<models::ActionWithOption>> {
    log::info!("db.action: get_action_by_id id={}", id);
    let mut stmt = self.conn().prepare(
      "select id, name, description, pattern, created_at, updated_at from actions where id = ?1",
    )?;
    match stmt.query_row(params![id], |row| Ok(models::Action::from_row(row))) {
      Ok(action) => {
        let action = action?;
        let options = self.get_options_by_action_id(action.id)?;
        Ok(Some(models::ActionWithOption {
          id: action.id,
          name: action.name,
          description: action.description,
          pattern: action.pattern,
          created_at: action.created_at,
          updated_at: action.updated_at,
          options,
        }))
      }
      Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub(super) fn get_actions(&self) -> Result<Vec<models::Action>> {
    log::info!("db.action: get_actions invoked");
    let mut stmt = self
      .conn()
      .prepare("select id, name, description, pattern, created_at, updated_at from actions")?;
    let actions = stmt.query_map(params![], |row| Ok(models::Action::from_row(row)))?;
    let mut result = Vec::new();
    for action in actions {
      result.push(action??);
    }
    Ok(result)
  }

  pub fn get_action_options(&self) -> Result<Vec<models::ActionWithOption>> {
    log::info!("db.action: get_action_options invoked");
    let mut stmt = self
      .conn()
      .prepare("select id, name, description, pattern, created_at, updated_at from actions")?;
    let actions = stmt.query_map(params![], |row| Ok(models::Action::from_row(row)))?;
    let mut result = Vec::new();
    for action in actions {
      let action = action??;
      let options = self.get_options_by_action_id(action.id)?;
      let action = models::ActionWithOption {
        id: action.id,
        name: action.name,
        description: action.description,
        pattern: action.pattern,
        created_at: action.created_at,
        updated_at: action.updated_at,
        options,
      };
      result.push(action);
    }
    Ok(result)
  }

  pub fn delete_action(&self, id: u64) -> Result<bool> {
    log::info!("db.action: delete_action invoked id={}", id);
    let rows_affected = self
      .conn()
      .execute("delete from actions where id = ?1", params![id])?;
    Ok(rows_affected > 0)
  }

  pub fn get_options_by_action_id(&self, id: u64) -> Result<Vec<models::RiboOption>> {
    log::info!("db.action: get_options_by_action_id invoked id={}", id);
    let mut stmt = self
      .conn()
      .prepare("select id, action_id, name, description, command, out, created_at, updated_at from options where action_id = ?1")?;
    let res = stmt.query_map(params![id], |row| Ok(models::RiboOption::from_row(row)))?;
    let mut options = vec![];
    for option in res {
      options.push(option??);
    }
    Ok(options)
  }

  pub fn create_option(&self, new_option: models::NewRiboOption) -> Result<models::RiboOption> {
    log::info!(
      "db.actions: create_option invoked (action_id={:?}, command={:?})",
      new_option.action_id,
      new_option.command
    );
    let mut stmt = self.conn().prepare("insert into options (action_id, command, description, out, name) values (?1, ?2, ?3, ?4, ?5) RETURNING id, action_id, name, description, command, out, created_at, updated_at")?;
    let option = stmt.query_row(
      params![
        new_option.action_id,
        new_option.command,
        new_option.description,
        new_option.out,
        new_option.name
      ],
      |row| Ok(models::RiboOption::from_row(row)),
    );
    option?
  }

  pub fn delete_option(&self, id: u64) -> Result<bool> {
    log::info!("db.action: delete_option invoked id={}", id);
    let rows_affected = self
      .conn()
      .execute("delete from options where id = ?1", params![id])?;
    Ok(rows_affected > 0)
  }

  pub fn update_action(&self, action: models::UpdateAction) -> Result<bool> {
    log::info!("db.actions: update_action invoked (id={:?})", action.id);
    let rows_affected = self.conn().execute(
      "update actions set description = ?1, pattern = ?2, name = ?3 where id = ?4",
      params![action.description, action.pattern, action.name, action.id],
    )?;
    Ok(rows_affected > 0)
  }

  pub fn update_option(&self, option: models::UpdateRiboOption) -> Result<bool> {
    log::info!("db.actions: update_option invoked (id={:?})", option.id);
    let rows_affected = self.conn().execute(
      "update options set command = ?1, description = ?2, out = ?3, name = ?4 where id = ?5",
      params![option.command, option.description, option.out, option.name, option.id],
    )?;
    Ok(rows_affected > 0)
  }

  pub fn get_option_by_id(&self, id: u64) -> Result<models::RiboOption> {
    log::info!("db.action: get_option_by_id invoked id={}", id);
    let mut stmt = self
      .conn()
      .prepare("select id, action_id, name, description, command, out, created_at, updated_at from options where id = ?1")?;
    let res = stmt.query_row(params![id], |row| Ok(models::RiboOption::from_row(row)))??;
    Ok(res)
  }
}

#[cfg(test)]
mod tests {
  use crate::{Database, models};

  fn init_data(db: &Database) {
    db.conn()
      .execute_batch(
        "BEGIN TRANSACTION; DELETE FROM actions; DELETE FROM sqlite_sequence WHERE name = 'actions';commit;")
      .unwrap();
  }
  #[test]
  fn test_create_action() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();
    init_data(&db);
    let action = db.create_action(models::NewAction {
      description: Some("test".to_string()),
      pattern: "*".to_string(),
      name: "test".to_string()
    });
    println!("{:?}", action);
    assert!(action.is_ok());
  }

  #[test]
  fn test_get_action_by_id() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();

    let action = db.get_action_by_id(1);
    println!("{:?}", action);
    assert!(action.is_ok());
  }

  #[test]
  fn test_get_actions() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();

    let action = db.get_actions();
    println!("{:?}", action);
    assert!(action.is_ok());
  }

  #[test]
  fn test_delete_action() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();

    let action = db.delete_action(1);
    println!("{:?}", action);
    assert!(action.is_ok());
  }
}
