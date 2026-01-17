use rusqlite::params;

use crate::{
  Database, Result,
  models::ActionWithOption,
};

impl Database {
  pub fn create_action_option_by_record(&mut self, record_id: u64, content: &str) -> Result<()> {
    log::info!(
      "db.record_action.create_action_option_by_record: (record_id={})",
      record_id
    );
    let actions = self.get_actions()?;
    let ids = actions
      .iter()
      .filter(|action| match regex::Regex::new(&action.pattern) {
        Ok(reg) => reg.is_match(content),
        Err(_) => false,
      })
      .map(|i| (record_id, i.id))
      .collect();

    self.create_record_actions(ids)?;
    Ok(())
  }
  pub fn create_action_option_by_action(&mut self, action_id: u64, pattern: &str) -> Result<()> {
    log::info!(
      "db.record_action.create_action_option_by_action: (action_id={})",
      action_id
    );
    let records = self.get_records()?;
    let ids = records
      .iter()
      .filter(|record| match regex::Regex::new(pattern) {
        Ok(reg) => reg.is_match(&record.content),
        Err(_) => false,
      })
      .map(|record| (record.id, action_id))
      .collect();

    self.create_record_actions(ids)?;
    Ok(())
  }
  pub fn update_action_option_by_record(&mut self, record_id: u64, content: &str) -> Result<()> {
    log::info!(
      "db.record_action.update_action_option_by_record: (record_id={})",
      record_id
    );
    {
      let mut stmt = self
        .conn()
        .prepare("delete from record_action where record_id=?")?;
      stmt.execute(params![record_id])?;
    }
    self.create_action_option_by_record(record_id, content)?;
    Ok(())
  }
  pub fn update_action_option_by_action(&mut self, action_id: u64, pattern: &str) -> Result<()> {
    log::info!(
      "db.record_action.update_action_option_by_action: (action_id={})",
      action_id
    );
    {
      let mut stmt = self
        .conn()
        .prepare("delete from record_action where action_id=?")?;
      stmt.execute(params![action_id])?;
    }
    self.create_action_option_by_action(action_id, pattern)?;
    Ok(())
  }
  fn create_record_actions(&mut self, ids: Vec<(u64, u64)>) -> Result<()> {
    log::info!(
      "db.record_action.create_record_actions: (len={})",
      ids.len()
    );
    let tx = self.conn_mut().transaction()?;
    {
      let mut stmt =
        tx.prepare("insert into record_action (record_id, action_id) values (?1, ?2);")?;
      for (record_id, action_id) in ids {
        stmt.execute(params![record_id, action_id])?;
      }
    }
    tx.commit()?;
    Ok(())
  }

  pub fn get_actions_by_record_id(&self, record_id: u64) -> Result<Vec<ActionWithOption>> {
    log::info!(
      "db.record_action.get_actions_by_record_id: (record_id={})",
      record_id
    );
    let mut stmt = self
      .conn()
      .prepare("select action_id from record_action where record_id = ?1;")?;
    let ids = stmt.query_map(params![record_id], |row| row.get(0))?;
    let mut actions = Vec::new();
    for id in ids {
      if let Some(action) = self.get_action_by_id(id?)? {
        actions.push(action);
      }
    }
    Ok(actions)
  }
}
