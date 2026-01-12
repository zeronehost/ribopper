-- record table
create table if not exists record (
  id integer primary key autoincrement,
  content text not null,
  data text not null,
  type text not null,
  created_at datetime default (DATETIME('now', 'localtime')),
  updated_at datetime default (DATETIME('now', 'localtime'))
);
create index if not exists idx_record_created_at on record (created_at desc);

-- record 触发器 - 使用 BEFORE UPDATE 避免循环
DROP TRIGGER IF EXISTS trigger_record_update_timestamp;
CREATE TRIGGER trigger_record_update_timestamp
  BEFORE UPDATE ON record
  FOR EACH ROW
BEGIN
  UPDATE record SET updated_at = DATETIME('now', 'localtime') 
  WHERE id = NEW.id;
END;