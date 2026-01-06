create table if not exists schema_version (
  version integer not null
);

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

-- -- target table
-- create table if not exists target (
--   id integer primary key autoincrement,
--   name text not null unique,  -- 添加唯一约束
--   description text,  -- 可选的描述字段
--   created_at datetime default (DATETIME('now', 'localtime')),
--   updated_at datetime default (DATETIME('now', 'localtime'))
-- );

-- -- target 触发器
-- DROP TRIGGER IF EXISTS trigger_target_update_timestamp;
-- CREATE TRIGGER trigger_target_update_timestamp
--   BEFORE UPDATE ON target
--   FOR EACH ROW
-- BEGIN
--   UPDATE target SET updated_at = DATETIME('now', 'localtime') 
--   WHERE id = NEW.id;
-- END;

-- -- record_target table (关联表)
-- create table if not exists record_target (
--   id integer primary key autoincrement,
--   target_id integer not null,
--   record_id integer not null,
--   created_at datetime default (DATETIME('now', 'localtime')),
--   updated_at datetime default (DATETIME('now', 'localtime')),
  
--   -- 添加外键约束
--   foreign key (target_id) references target(id) 
--     on delete cascade,  -- 删除target时自动删除关联记录
  
--   foreign key (record_id) references record(id)
--     on delete cascade,  -- 删除record时自动删除关联记录
  
--   -- 添加唯一约束，防止重复关联
--   unique (target_id, record_id)
-- );

-- -- 创建复合索引（如果经常同时查询）
-- create index if not exists idx_record_target_composite on record_target (target_id, record_id);
-- create index if not exists idx_record_target_record on record_target (record_id);

-- -- record_target 触发器
-- DROP TRIGGER IF EXISTS trigger_record_target_update_timestamp;
-- CREATE TRIGGER trigger_record_target_update_timestamp
--   BEFORE UPDATE ON record_target
--   FOR EACH ROW
-- BEGIN
--   UPDATE record_target SET updated_at = DATETIME('now', 'localtime') 
--   WHERE id = NEW.id;
-- END;

-- actions table
CREATE TABLE IF NOT EXISTS actions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  description TEXT,
  pattern TEXT NOT NULL,
  -- options TEXT NOT NULL,  -- 存储为 JSON 字符串
  created_at datetime default (DATETIME('now', 'localtime')),
  updated_at datetime default (DATETIME('now', 'localtime'))
);

-- actions 触发器
DROP TRIGGER IF EXISTS trigger_action_update_timestamp;
CREATE TRIGGER trigger_actions_update_timestamp
  BEFORE UPDATE ON actions
  FOR EACH ROW
BEGIN
  UPDATE actions SET updated_at = DATETIME('now', 'localtime') 
  WHERE id = NEW.id;
END;

-- options
DROP TABLE IF EXISTS options;
CREATE TABLE IF NOT EXISTS options (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  command TEXT NOT NULL,
  description TEXT,
  out char(1) NOT NULL,
  action_id INTEGER NOT NULL,
  created_at datetime default (DATETIME('now', 'localtime')),
  updated_at datetime default (DATETIME('now', 'localtime')),

  -- 添加外键约束
  FOREIGN KEY (action_id) REFERENCES actions(id) ON DELETE CASCADE
);

-- options 触发器
DROP TRIGGER IF EXISTS trigger_options_update_timestamp;
CREATE TRIGGER trigger_options_update_timestamp
  BEFORE UPDATE ON options
  FOR EACH ROW
BEGIN
  UPDATE options SET updated_at = DATETIME('now', 'localtime') 
  WHERE id = NEW.id;
END;