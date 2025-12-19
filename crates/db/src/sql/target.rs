use rusqlite::params;

use crate::{
  Result,
  models::{self, FromRow},
};

use super::db::Database;

impl Database {
  pub fn create_target(&self, target: models::NewTarget) -> Result<models::Target> {
    let mut stmt = self.conn().prepare(
      r#"
    INSERT INTO target (name, description)
    VALUES (?1, ?2)
    RETURNING id, name, description, created_at, updated_at
    "#,
    )?;
    
    stmt.query_row(params![target.name, target.description], |row| {
      Ok(models::Target::from_row(row))
    })?
  }

  pub fn get_target_by_name(&self, name: &str) -> Result<Option<models::Target>> {
    let mut stmt = self
      .conn()
      .prepare("select * from target where name = ?1")?;
    match stmt.query_row(params![name], |row| Ok(models::Target::from_row(row))) {
      Ok(target) => Ok(Some(target?)),
      Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub fn get_targets(&self) -> Result<Vec<models::Target>> {
    let mut stmt = self.conn().prepare("select * from target order by name")?;
    let targets = stmt.query_map([], |row| Ok(models::Target::from_row(row)))?;
    let mut res = Vec::new();
    for target in targets {
      res.push(target??);
    }
    Ok(res)
  }

  pub fn delete_target(&self, id: i64) -> Result<()> {
    let mut stmt = self.conn().prepare("delete from target where id = ?1")?;
    stmt.execute(params![id])?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  fn init_data(db: &Database) {
    db.conn().execute_batch("BEGIN TRANSACTION; DELETE FROM target; DELETE FROM sqlite_sequence WHERE name = 'target';commit;").unwrap();
  }
  #[test]
  fn test_create() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    let result = db.create_target(models::NewTarget {
      name: "test_target".to_string(),
      description: Some("This is a test target".to_string()),
    });
    println!("{:?}", result);
    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.name, "test_target");
  }

  #[test]
  fn test_get_by_name() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建测试数据
    let created = db
      .create_target(models::NewTarget {
        name: "test_target1".to_string(),
        description: Some("This is a test target".to_string()),
      })
      .unwrap();

    // 测试获取存在的记录
    let found = db.get_target_by_name("test_target1").unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, created.id);

    // 测试获取不存在的记录
    let not_found = db.get_target_by_name("nonexistent").unwrap();
    assert!(not_found.is_none());
  }

  #[test]
  fn test_get_all() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建多个测试记录
    for i in 0..3 {
      db.create_target(models::NewTarget {
        name: format!("target_{}", i),
        description: Some(format!("This is target {}", i)),
      })
      .unwrap();
    }

    // 获取所有记录
    let all = db.get_targets().unwrap();
    assert!(all.len() >= 3);

    // 验证排序
    for window in all.windows(2) {
      assert!(window[0].name <= window[1].name);
    }
  }

  #[test]
  fn test_duplicate_name() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建第一个记录
    db.create_target(models::NewTarget {
      name: "duplicate".to_string(),
      description: Some("This is a test target".to_string()),
    })
    .unwrap();

    // 尝试创建同名记录
    let result = db.create_target(models::NewTarget {
      name: "duplicate".to_string(),
      description: Some("This is another test target".to_string()),
    });

    assert!(result.is_err());
  }
}
