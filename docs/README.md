# 剪贴板

## 系统托盘

### 左键窗口

1. 窗口标题栏

    1. 退出图标
    2. 标题
    3. 搜索
    4. 清空图标
    5. 设置图标
    6. 固定图标
    
2. 内容区域
    
    若有收藏，则显示收藏面板，以tabs形式展示
    
    1. 剪贴板内容列表
        
        1. 列表项
        
            1. 默认显示剪贴板内容
            2. focus 时展示操作区域
                
                1. 执行 TODO
                2. 二维码
                3. 编辑
                4. 收藏
                5. 删除
        
    2. 收藏列表（同内容列表）

### 右键菜单

1. 清空历史记录
    
    弹框提醒
    
2. 设置
3. 关于
4. 退出

## 窗口

### 通用配置

1. 历史记录数量
    
    默认 20

### 操作配置

TODO
    
### 快捷键设置

1. 编辑内容
2. 清除历史记录
3. 上一条记录
4. 下一条记录
5. 显示二维码
6. 显示剪贴板（当前鼠标位置）

----------------

# 数据库设计

## 剪贴板内容

### 字段

- id 条目 ID
- content 剪贴板内容
- type 类型
- created_at 创建时间
- updated_at 更新时间
- is_favorite 是否收藏

一下根据 type 字段，展示不同字段
- deletable 是否可删除
- editable 是否可编辑
- executable 是否可执行
- scannable 是否可扫描
- starable 是否可收藏

<!-- 1. 剪贴板内容

    ```sql
    CREATE TABLE clipboard (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        content TEXT NOT NULL,
        type: INTEGER DEFAULT 0, -- 类型
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        is_favorite INTEGER DEFAULT 0, -- 是否收藏
        deletable INTEGER DEFAULT 0, -- 是否可删除
        editable INTEGER DEFAULT 0, -- 是否可编辑
        executable INTEGER DEFAULT 0, -- 是否可执行
        scannable INTEGER DEFAULT 0, -- 是否可扫描
        starable INTEGER DEFAULT 0, -- 是否可收藏
    );
    ``` -->



-----------------------------------------------------

# 依赖

## rust

1. qrcode

    ```sh
    cargo install qrcode
    ```
    https://docs.rs/qrcode/latest/qrcode/
    
2. 全局快捷方式

    https://tauri.app/zh-cn/plugin/global-shortcut/
    
3. SQL

    https://tauri.app/zh-cn/plugin/sql/
    
4. 指定打开应用
    
    https://tauri.app/zh-cn/plugin/opener/
    
5. 单例

    https://tauri.app/zh-cn/plugin/single-instance/

6. 日志

    https://tauri.app/zh-cn/plugin/logging/
    
7. 剪贴板

    https://tauri.app/zh-cn/plugin/clipboard/
    arboard
    
8. 自动启动

    https://tauri.app/zh-cn/plugin/autostart/
    
9. 更新

    https://tauri.app/zh-cn/plugin/updater/

10. 组件库

    https://soberjs.com/
