use super::db::Database;
use crate::{
  error::Result,
  models::{self, FromRow},
};
use rusqlite::{ToSql, params};

impl Database {
  pub fn create_record(&self, record: models::NewRecord) -> Result<models::Record> {
    let mut stmt = self.conn().prepare("insert into record (content, data, type) values (?1, ?2, ?3) RETURNING id, content, data, type, created_at, updated_at")?;
    let res = stmt.query_row(params![record.content, record.data, record.typ], |row| {
      Ok(models::Record::from_row(row))
    })?;
    Ok(res?)
  }

  pub fn get_record_by_id(&self, id: u64) -> Result<Option<models::Record>> {
    let mut stmt = self.conn().prepare("select * from record where id = ?1;")?;
    match stmt.query_row(params![id], |row| Ok(models::Record::from_row(row))) {
      Ok(record) => Ok(Some(record?)),
      Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub fn update_record_content(&self, id: u64, content: String) -> Result<bool> {
    let rows_affected = self.conn().execute(
      "update record set content = ?1 where id = ?2",
      params![content, id],
    )?;
    Ok(rows_affected > 0)
  }

  pub fn delete_record(&self, id: u64) -> Result<bool> {
    let rows_affected = self
      .conn()
      .execute("delete from record where id = ?1", params![id])?;
    Ok(rows_affected > 0)
  }

  pub fn query_record(&self, query: models::RecordQuery) -> Result<Vec<models::RecordWithTargets>> {
    let mut sql = String::from(
      r#"
      SELECT 
          c.id as record_id,
          c.content,
          c.data,
          c.type,
          c.created_at as record_created,
          c.updated_at as record_updated,
          COALESCE(json_group_array(t.name), '[]') as target_names,
          COUNT(ct.target_id) as target_count
      FROM record c
      LEFT JOIN record_target ct ON c.id = ct.record_id
      LEFT JOIN target t ON ct.target_id = t.id
      WHERE 1=1
      "#,
    );
    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    let mut conditions = Vec::new();

    // 构建查询条件
    if let Some(ref content) = query.content_contains {
      conditions.push("c.content LIKE ?");
      params.push(Box::new(format!("%{}%", content)));
    }
    if let Some(target_id) = query.target_id {
      conditions
        .push("EXISTS (SELECT 1 FROM record_target WHERE record_id = c.id AND target_id = ?)");
      params.push(Box::new(target_id));
    }
    if let Some(ref start_date) = query.start_date {
      conditions.push("c.created_at >= ?");
      params.push(Box::new(start_date.format("%Y-%m-%d %H:%M:%S").to_string()));
    }

    if let Some(ref end_date) = query.end_date {
      conditions.push("c.created_at <= ?");
      params.push(Box::new(end_date.format("%Y-%m-%d %H:%M:%S").to_string()));
    }

    if !conditions.is_empty() {
      sql.push_str(" AND ");
      sql.push_str(&conditions.join(" AND "));
    }

    // 分组
    sql.push_str(" GROUP BY c.id");

    // 排序
    let order_by = query.order_by.unwrap_or_else(|| "c.created_at".to_string());
    let order_direction = query.order_direction.unwrap_or_else(|| "DESC".to_string());
    sql.push_str(&format!(" ORDER BY {} {}", order_by, order_direction));

    // 分页
    if let Some(limit) = query.limit {
      sql.push_str(&format!(" LIMIT {}", limit));

      if let Some(offset) = query.offset {
        sql.push_str(&format!(" OFFSET {}", offset));
      }
    }

    // 执行查询
    let mut stmt = self.conn().prepare(&sql)?;
    let param_refs: Vec<&dyn ToSql> = params.iter().map(|p| &**p).collect();

    let rows = stmt.query_map(rusqlite::params_from_iter(param_refs), |row| {
      Ok(models::RecordWithTargets::from_row(row))
    })?;

    let mut results = Vec::new();
    for row in rows {
      results.push(row??);
    }

    Ok(results)
  }

  pub fn get_record_recent(&self, limit: i64) -> Result<Vec<models::Record>> {
    let mut stmt = self
      .conn()
      .prepare("select * from record order by created_at desc limit ?1;")?;
    let record_iter = stmt.query_map(params![limit], |row| Ok(models::Record::from_row(row)))?;
    let mut results = Vec::new();
    for record in record_iter {
      results.push(record??);
    }
    Ok(results)
  }

  pub fn batch_inset_record(&self, records: Vec<models::NewRecord>) -> Result<Vec<models::Record>> {
    let mut results = Vec::with_capacity(records.len());

    for record in records {
      let result = self.create_record(record)?;
      results.push(result);
    }

    Ok(results)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  fn init_data(db: &Database) {
    db.conn()
      .execute_batch(
        "BEGIN TRANSACTION; DELETE FROM record; DELETE FROM sqlite_sequence WHERE name = 'record';commit;")
      .unwrap();
  }
  #[test]
  fn test_create() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");

    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    let record = db.create_record(models::NewRecord {
      content: "test content".to_string(),
      data: "test data".to_string(),
      typ: models::RecordType::Text,
    });

    println!("record: {:?}", record);

    assert!(record.is_ok());
  }

  #[test]
  fn test_get_by_id() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建测试数据
    let created = db
      .create_record(models::NewRecord {
        content: "test content".to_string(),
        data: "test data".to_string(),
        typ: models::RecordType::Text,
      })
      .unwrap();

    // 测试获取存在的记录
    let found = db.get_record_by_id(created.id).unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().content, "test content");

    // 测试获取不存在的记录
    let not_found = db.get_record_by_id(999999).unwrap();
    assert!(not_found.is_none());
  }

  #[test]
  fn test_update_content() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建测试数据
    let created = db
      .create_record(models::NewRecord {
        content: "original content".to_string(),
        data: "test data".to_string(),
        typ: models::RecordType::Text,
      })
      .unwrap();

    // 更新内容
    let updated = db
      .update_record_content(created.id, "updated content".to_string())
      .unwrap();
    assert!(updated);

    // 验证更新
    let updated_record = db.get_record_by_id(created.id).unwrap().unwrap();
    assert_eq!(updated_record.content, "updated content");
  }

  #[test]
  fn test_delete() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建测试数据
    let created = db
      .create_record(models::NewRecord {
        content: "to be deleted".to_string(),
        data: "test data".to_string(),
        typ: models::RecordType::Text,
      })
      .unwrap();

    // 删除记录
    let deleted = db.delete_record(created.id).unwrap();
    assert!(deleted);

    // 验证删除
    let found = db.get_record_by_id(created.id).unwrap();
    assert!(found.is_none());
  }

  #[test]
  fn test_get_recent() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 创建多个测试记录
    for i in 0..5 {
      db.create_record(models::NewRecord {
        content: format!("content {}", i),
        data: format!("data {}", i),
        typ: models::RecordType::Text,
      })
      .unwrap();
    }

    // 获取最近的记录
    let recent = db.get_record_recent(3).unwrap();
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0].content, "content 0");
  }

  #[test]
  fn test_batch_insert() {
    let uri = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test.db");
    let db = Database::new(uri, None).unwrap();
    init_data(&db);

    // 准备批量插入数据
    let records: Vec<models::NewRecord> = (0..3)
      .map(|i| models::NewRecord {
        content: format!("batch content {}", i),
        data: format!("batch data {}", i),
        typ: models::RecordType::Text,
      })
      .collect();

    // 执行批量插入
    let results = db.batch_inset_record(records).unwrap();
    assert_eq!(results.len(), 3);

    // 验证插入结果
    for (i, result) in results.iter().enumerate() {
      assert_eq!(result.content, format!("batch content {}", i));
    }
  }
}
