create table if not exists ribo_schema (version interger not null);

-- history table
create table if not exists history (
  id integer primary key autoincrement,
  content text not null,
  type interger not null,
  favorites interger default 0,
  created_at datetime default (DATETIME('now', 'localtime')),
  updated_at datetime default (DATETIME('now', 'localtime'))
);

-- 触发器
DROP TRIGGER IF EXISTS trigger_update_timestamp;

CREATE TRIGGER trigger_update_timestamp
  AFTER UPDATE
    ON history
BEGIN
    UPDATE history
       SET updated_at = DATETIME('now', 'localtime') 
     WHERE rowid = NEW.rowid;
END;