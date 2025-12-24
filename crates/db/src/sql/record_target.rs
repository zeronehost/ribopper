use super::db::Database;
use crate::{
  error::Result,
  models::{self, FromRow},
};
use rusqlite::params;

impl Database {
  pub fn create_record_target(
    &self,
    record_target: &models::NewRecordTarget,
  ) -> Result<models::RecordTarget> {
    log::info!(
      "db.record_target: create_record_target record_id={} target_id={}",
      record_target.record_id,
      record_target.target_id
    );
    let mut stmt = self.conn().prepare(
      r#"
      INSERT INTO record_target (target_id, record_id)
      VALUES (?, ?)
      RETURNING id, target_id, record_id, created_at, updated_at
      "#,
    )?;

    stmt.query_row(
      params![record_target.target_id, record_target.record_id],
      |row| Ok(models::RecordTarget::from_row(row)),
    )?
  }

  pub fn delete_record_target(&self, id: u64) -> Result<bool> {
    log::info!("db.record_target: delete_record_target id={}", id);
    let rows_affected = self
      .conn()
      .execute("delete from record_target where id = ?1", params![id])?;
    log::debug!("db.record_target: delete affected {} rows", rows_affected);
    Ok(rows_affected > 0)
  }

  pub fn get_record_target_by_record_id(
    &self,
    record_id: i64,
  ) -> Result<Vec<models::RecordTarget>> {
    log::debug!(
      "db.record_target: get_record_target_by_record_id record_id={}",
      record_id
    );
    let mut stmt = self
      .conn()
      .prepare("SELECT * FROM record_target WHERE record_id = ?1 ORDER BY created_at")?;

    let rows = stmt.query_map(params![record_id], |row| {
      Ok(models::RecordTarget::from_row(row))
    })?;

    let mut record_targets = Vec::new();
    for row in rows {
      record_targets.push(row??);
    }
    Ok(record_targets)
  }

  pub fn get_record_target_by_target_id(
    &self,
    target_id: u64,
  ) -> Result<Vec<models::RecordTarget>> {
    log::debug!(
      "db.record_target: get_record_target_by_target_id target_id={}",
      target_id
    );
    let mut stmt = self
      .conn()
      .prepare("SELECT * FROM record_target WHERE target_id = ?1 ORDER BY created_at")?;
    let rows = stmt.query_map(params![target_id], |row| {
      Ok(models::RecordTarget::from_row(row))
    })?;

    let mut record_targets = Vec::new();
    for row in rows {
      record_targets.push(row??);
    }
    Ok(record_targets)
  }

  pub fn clear_records(&self) -> Result<()> {
    self.conn().execute_batch(
      r#"
    begin transaction;
    delete from record_target;
    delete from record;
    delete from sqlite_sequence where name = 'record_target';
    delete from sqlite_sequence where name = 'record';
    commit"#,
    )?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {

  use crate::models;

  use super::*;
  fn init_data(db: &Database) {
    db.conn().execute_batch("BEGIN TRANSACTION; DELETE FROM target; DELETE FROM sqlite_sequence WHERE name = 'target'; DELETE FROM record; DELETE FROM sqlite_sequence WHERE name = 'record'; delete from record_target; commit").unwrap();

    for i in 0..10 {
      db.create_record(
        models::NewRecord {
          content: format!("record {}", i),
          data: format!("record data {}", i),
          typ: models::RecordType::Text,
        },
        None,
      )
      .unwrap();
      db.create_target(models::NewTarget {
        name: format!("target_{}", i),
        description: Some(format!("target data {}", i)),
      })
      .unwrap();
    }
  }
  #[test]
  fn test_create() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    let result = db.create_record_target(&models::NewRecordTarget {
      target_id: 1,
      record_id: 1,
    });
    println!("result: {:?}", result);
    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.target_id, 1);
    assert_eq!(created.record_id, 1);
  }

  #[test]
  fn test_delete() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建测试数据
    let created = db
      .create_record_target(&models::NewRecordTarget {
        target_id: 1,
        record_id: 2,
      })
      .unwrap();

    // 删除记录
    let deleted = db.delete_record_target(created.id).unwrap();
    assert!(deleted);

    // 验证删除
    let not_deleted = db.delete_record_target(created.id).unwrap();
    assert!(!not_deleted);
  }

  #[test]
  fn test_get_by_record_id() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建多个测试记录
    for i in 1..4 {
      db.create_record_target(&models::NewRecordTarget {
        target_id: i,
        record_id: 1,
      })
      .unwrap();
    }

    // 获取记录
    let results = db.get_record_target_by_record_id(1).unwrap();
    assert_eq!(results.len(), 3);

    // 验证所有记录的record_id都正确
    for result in results {
      assert_eq!(result.record_id, 1);
    }
  }

  #[test]
  fn test_get_by_target_id() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建多个测试记录
    for i in 1..4 {
      db.create_record_target(&models::NewRecordTarget {
        target_id: 1,
        record_id: i,
      })
      .unwrap();
    }

    // 获取记录
    let results = db.get_record_target_by_target_id(1).unwrap();
    assert_eq!(results.len(), 3);

    // 验证所有记录的target_id都正确
    for result in results {
      assert_eq!(result.target_id, 1);
    }
  }

  #[test]
  fn test_empty_results() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 测试获取不存在的记录
    let empty_results = db.get_record_target_by_record_id(999).unwrap();
    assert!(empty_results.is_empty());

    let empty_results = db.get_record_target_by_target_id(999).unwrap();
    assert!(empty_results.is_empty());
  }
}
