# 外部集成

**分析日期:** 2026-03-17

## API 与外部服务

**桌面框架:**
- Tauri 2 - 桌面应用框架
  - 通信方式: `@tauri-apps/api/core::invoke()` RPC 调用
  - 前端入口: `src/lib/api.ts`
  - 后端入口: `src-tauri/src/commands/mod.rs`

**无需外部 API:**
本项目为桌面应用，不依赖外部云服务 API。所有数据存储在本地 SQLite 数据库中。

## 数据存储

**数据库:**
- SQLite (bundled)
  - 库: `rusqlite` 0.31
  - 存储位置: `{app_data_dir}/projectflow.db`
  - 软删除: 使用 `deleted_at` 时间戳字段实现

**数据库表:**
- `projects` - 项目表
- `tasks` - 任务表
- `documents` - 文档表
- `milestones` - 里程碑表
- `users` - 用户表
- `task_dependencies` - 任务依赖关系表

**文件存储:**
- 本地文件系统 - 文档内容存储在数据库 `content` 字段
- 无外部云存储集成

**缓存:**
- 无缓存层 - 直接操作 SQLite 数据库

## 身份验证

**认证方式:**
- 本地应用 - 无需身份验证
- 无外部身份提供商集成
- 应用数据存储在本地

## 监控与可观测性

**错误追踪:**
- 无外部错误追踪服务

**日志:**
- `log` 0.4 + `env_logger` 0.11 (Rust 后端)
- 浏览器控制台 (前端)

## CI/CD 与部署

**托管平台:**
- GitHub Releases - 应用分发
- 本地安装 - 用户自行下载安装

**CI/CD 流水线:**
- GitHub Actions - 自动化构建与发布
  - 配置文件: `.github/workflows/release.yml`
  - 触发条件: 版本标签 (v*)
  - 构建目标:
    - Windows (x64): `windows-latest`
    - macOS (x64 + ARM): `macos-latest`
  - 发布方式: GitHub Draft Release

**构建流程:**
```bash
# 前端
npm run build  # vue-tsc + vite build

# Tauri
npm run tauri build  # 构建桌面应用
```

## 环境配置

**无需环境变量:**
- 应用为桌面客户端，无需外部环境配置
- 数据库路径由 Tauri 自动管理

## Webhooks 与回调

**无外部 webhook:**
- 不接受外部 webhook
- 不发送 outbound webhook

---

*外部集成审计: 2026-03-17*
