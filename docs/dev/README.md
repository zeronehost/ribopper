# 开发文档

## 项目概述
ribo-pper是一个基于Tauri和Vue.js的跨平台剪贴板管理工具，采用Rust后端和TypeScript前端的混合架构。

## 开发环境搭建

### 前端开发环境
1. 安装Node.js (v24.x) 和pnpm
2. 安装依赖
```bash
pnpm install
```
3. 启动开发服务器

```bash
cargo tauri dev
```

## 项目结构详解

### 核心目录结构
```
crates/
├── app/          # Tauri主应用
├── clipboard/    # 剪贴板处理模块
└── db/           # 数据库操作模块
packages/
├── api/          # API定义
└── app/          # Vue前端应用
```

## 开发规范

### 代码规范

- 前端遵循 oxlint 配置，使用 oxlint 格式化
- 后端遵循Rustfmt配置
- 提交信息使用Conventional Commits规范

### 分支管理

- `main`: 主分支，保持可部署状态
- `dev`: 开发分支，功能开发完成后合并到此
- `feature/*`: 功能分支，新功能开发

## 测试指南


### 后端测试
```bash
cargo test
```

## 常见问题

### 构建失败
- 确保所有依赖已安装
- 检查Tauri环境依赖是否完整
- 清理缓存后重试
```bash
cargo clean
pnpm clean
```

### 开发服务器无法启动
- 检查端口是否被占用
- 检查Node.js版本是否兼容

## 构建

### 构建应用
```bash
cargo tauri build
```
