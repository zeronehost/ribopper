-- actions table
CREATE TABLE IF NOT EXISTS actions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  description TEXT,
  pattern TEXT NOT NULL,
  name text NOT NULL,
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
CREATE TABLE IF NOT EXISTS options (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
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

-- record_action table
create table if not exists record_action (
  id integer primary key autoincrement,
  record_id integer not null,
  action_id integer not null,

  -- 添加外键约束
  foreign key (record_id) references record(id) on delete cascade,  -- 删除record时自动删除关联记录
  foreign key (action_id) references actions(id) on delete cascade,  -- 删除action时自动删除关联记录

  -- 添加唯一约束，防止重复关联
  unique (record_id, action_id)
);