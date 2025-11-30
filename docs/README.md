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
