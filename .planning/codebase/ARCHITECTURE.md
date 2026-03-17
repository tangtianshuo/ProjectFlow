# 架构

**分析日期:** 2026-03-17

## 模式概述

**整体架构:** Tauri v2 + Vue 3 的前后端分离桌面应用架构

**关键特性:**
- 前端采用 Vue 3 Composition API + TypeScript
- 后端采用 Rust + Tauri v2 框架
- 状态管理使用 Pinia
- SQLite 数据库（本地存储）
- IPC 通信通过 Tauri invoke 调用

## 层次

**前端层 (Vue 3 + TypeScript):**
- 位置: `src/`
- 职责: 用户界面、交互逻辑、状态管理
- 依赖: `@tauri-apps/api/core`, `pinia`, `vue`
- 被后端层使用

**API 抽象层:**
- 位置: `src/lib/api.ts`
- 职责: 封装 Tauri invoke 调用，提供类型安全的 API 函数
- 导出: `projectApi`, `taskApi`, `documentApi`, `milestoneApi`
- 被组件和 Store 使用

**状态管理层 (Pinia):**
- 位置: `src/stores/`
- 职责: 应用状态管理、业务逻辑处理
- Store: `projectStore` (项目和任务), `uiStore` (UI 状态), `documentStore` (文档)
- 被组件使用

**组件层:**
- 位置: `src/components/`
- 职责: UI 渲染和用户交互
- 包含: UI 组件 (`ui/`), 特性组件 (`features/`), 布局组件 (`layout/`)

**后端层 (Rust + Tauri):**
- 位置: `src-tauri/src/`
- 职责: 业务逻辑处理、数据库操作
- 包含: 命令处理 (`commands/`), 数据库操作 (`db/`), 数据模型 (`models/`)

**数据库层 (SQLite):**
- 位置: `src-tauri/src/db/mod.rs`
- 职责: 数据持久化、CRUD 操作、软删除支持
- 客户端: `rusqlite`

## 数据流

**典型数据流 - 创建项目:**

1. 用户在 `ProjectList.vue` 组件中填写表单
2. 组件调用 `projectStore.createProject(data)`
3. `projectStore` 调用 `projectApi.create(data)`
4. `projectApi` 使用 `invoke("create_project", {...})` 调用 Rust 后端
5. Rust 命令 `create_project` 在 `commands/mod.rs` 中接收请求
6. 命令调用 `db.create_project()` 执行 SQLite 插入
7. 数据库返回创建的 Project 对象
8. 响应逐层返回，Store 更新状态
9. 组件响应式更新 UI

**状态管理数据流:**

```
App.vue (根组件)
    |
    v
useUiStore() -> currentView (控制显示哪个特性组件)
useProjectStore() -> projects[], tasks[], currentProject
    |
    v
UI 组件 (Dashboard, ProjectList, TaskBoard, DocumentCenter, RecycleBin)
```

## 关键抽象

**API 抽象:**
- 目的: 封装 Tauri IPC 调用，提供类型安全接口
- 示例: `src/lib/api.ts`
- 模式: 模块导出对象 (projectApi, taskApi, documentApi, milestoneApi)

**Store 抽象 (Pinia):**
- 目的: 集中管理应用状态和业务逻辑
- 示例: `src/stores/projectStore.ts`
- 模式: Composition API 风格的 defineStore，使用 ref/reactive 存储状态

**数据库抽象:**
- 目的: 封装 SQLite 操作
- 示例: `src-tauri/src/db/mod.rs`
- 模式: Database struct 封装 Connection，使用 Mutex 保证线程安全

**数据模型:**
- 目的: 前后端共享的数据结构定义
- 前端: `src/lib/api.ts` (TypeScript 接口)
- 后端: `src-tauri/src/models/mod.rs` (Rust struct)

## 入口点

**前端入口:**
- 位置: `src/main.ts`
- 职责: 初始化 Vue 应用、注册 Pinia、安装全局样式
- 触发: Vite 开发服务器或生产构建

**根组件:**
- 位置: `src/App.vue`
- 职责: 路由视图切换（基于 uiStore.currentView）、布局结构
- 触发: main.ts 挂载 App

**Rust 入口:**
- 位置: `src-tauri/src/lib.rs`
- 职责: 初始化 Tauri 应用、设置数据库、注册命令处理器
- 触发: Tauri 应用启动

**数据库初始化:**
- 位置: `src-tauri/src/db/mod.rs` -> `init_tables()`
- 职责: 创建所有表结构、索引、迁移

## 错误处理

**策略:** 前后端分离的错误传播

**前端模式:**
```typescript
try {
  await projectStore.createProject(data);
} catch (e) {
  error.value = String(e);
  // 显示错误提示
}
```

**后端模式:**
```rust
pub fn create_project(...) -> Result<Project, String> {
    db.create_project(...)
        .map_err(|e| e.to_string())  // 转换为字符串错误
}
```

## 跨领域关注点

**日志:** 使用 `env_logger` (Rust 端), `console` (Vue 端)

**验证:** 前端表单验证（必填字段检查），后端参数类型检查

**认证:** 当前为本地应用，未实现用户认证系统

**软删除:** 所有实体支持 deleted_at 字段，删除时标记而非物理删除

---

*架构分析: 2026-03-17*
