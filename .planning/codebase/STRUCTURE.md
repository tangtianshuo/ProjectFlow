# 代码库结构

**分析日期:** 2026-03-17

## 目录布局

```
project-root/
├── src/                        # 前端源码 (Vue 3 + TypeScript)
│   ├── main.ts                 # 前端入口
│   ├── App.vue                 # 根组件
│   ├── style.css               # 全局样式
│   ├── vite-env.d.ts           # Vite 类型声明
│   ├── assets/                 # 静态资源
│   ├── lib/                    # 库和工具
│   │   └── api.ts              # API 抽象层 (Tauri invoke 封装)
│   ├── stores/                 # Pinia 状态管理
│   │   ├── projectStore.ts     # 项目和任务状态
│   │   ├── documentStore.ts    # 文档状态
│   │   └── uiStore.ts          # UI 状态 (当前视图、侧边栏)
│   └── components/             # Vue 组件
│       ├── ui/                 # 通用 UI 组件
│       │   ├── Button.vue       # 按钮组件
│       │   ├── Input.vue        # 输入框组件
│       │   ├── Modal.vue        # 模态框组件
│       │   ├── Select.vue       # 选择器组件
│       │   ├── Icon.vue         # 图标组件
│       │   ├── Button.test.ts   # 按钮测试
│       │   └── Input.test.ts    # 输入框测试
│       ├── features/            # 特性模块组件
│       │   ├── dashboard/       # 仪表盘
│       │   │   └── Dashboard.vue
│       │   ├── projects/        # 项目管理
│       │   │   └── ProjectList.vue
│       │   ├── tasks/           # 任务管理
│       │   │   ├── TaskBoard.vue
│       │   │   └── GanttChart.vue
│       │   ├── documents/       # 文档中心
│       │   │   └── DocumentCenter.vue
│       │   └── recycleBin/      # 回收站
│       │       └── RecycleBin.vue
│       └── layout/              # 布局组件
│           └── Sidebar.vue      # 侧边栏导航
├── src-tauri/                  # 后端源码 (Rust + Tauri)
│   ├── src/
│   │   ├── main.rs             # Rust 入口 (移动端)
│   │   ├── lib.rs              # Rust 主入口
│   │   ├── commands/           # Tauri 命令处理
│   │   │   └── mod.rs          # 所有命令定义
│   │   ├── db/                 # 数据库层
│   │   │   └── mod.rs          # SQLite 操作
│   │   └── models/             # 数据模型
│   │       └── mod.rs          # Rust struct 定义
│   ├── Cargo.toml              # Rust 依赖
│   ├── tauri.conf.json         # Tauri 配置
│   ├── capabilities/           # Tauri 权限配置
│   └── icons/                  # 应用图标
├── .github/workflows/           # CI/CD 配置
├── package.json                # Node 依赖
├── tsconfig.json               # TypeScript 配置
├── vite.config.ts              # Vite 配置
└── index.html                  # HTML 入口
```

## 目录用途

**src/ - 前端源码**
- 所有 Vue 组件、TypeScript 代码
- 使用 Vite 构建

**src/lib/ - 库和工具**
- 包含 `api.ts`: Tauri invoke 封装
- 类型定义 (TypeScript 接口)
- API 函数导出

**src/stores/ - Pinia 状态管理**
- `projectStore.ts`: 项目、任务 CRUD 和状态
- `documentStore.ts`: 文档状态管理
- `uiStore.ts`: 当前视图、侧边栏折叠状态

**src/components/ui/ - 通用 UI 组件**
- 可复用的基础组件
- 遵循统一的设计规范
- 包含对应的单元测试

**src/components/features/ - 特性模块**
- 按功能模块划分的业务组件
- 每个模块独立的页面级组件
- 包含: dashboard, projects, tasks, documents, recycleBin

**src/components/layout/ - 布局组件**
- 应用整体布局结构
- 包含 Sidebar 导航组件

**src-tauri/src/ - Rust 后端**
- 所有 Rust 代码
- 数据库操作、命令处理、数据模型

## 关键文件位置

**入口点:**
- `src/main.ts` - 前端 Vue 应用入口
- `src-tauri/src/lib.rs` - Rust Tauri 应用入口
- `index.html` - HTML 入口

**配置:**
- `package.json` - Node 依赖和脚本
- `src-tauri/Cargo.toml` - Rust 依赖
- `src-tauri/tauri.conf.json` - Tauri 配置
- `tsconfig.json` - TypeScript 配置
- `vite.config.ts` - Vite 构建配置

**API 层:**
- `src/lib/api.ts` - 前端 API 封装

**状态管理:**
- `src/stores/projectStore.ts`
- `src/stores/uiStore.ts`
- `src/stores/documentStore.ts`

**数据库:**
- `src-tauri/src/db/mod.rs` - SQLite 操作
- `src-tauri/src/models/mod.rs` - 数据模型

**命令处理:**
- `src-tauri/src/commands/mod.rs` - 所有 Tauri 命令

## 命名约定

**文件:**
- Vue 组件: PascalCase (如 `ProjectList.vue`, `TaskBoard.vue`)
- TypeScript 文件: camelCase (如 `api.ts`, `projectStore.ts`)
- Rust 文件: snake_case (如 `mod.rs`, `lib.rs`)

**目录:**
- 组件目录: kebab-case (如 `project-list/`, `task-board/`)
- 模块目录: camelCase (如 `stores/`, `lib/`)

**TypeScript/JS:**
- 函数: camelCase (如 `createProject`, `fetchProjects`)
- 组件导入: PascalCase (如 `import Button from '...'`)
- 常量: UPPER_SNAKE_CASE (如 `VIEW_MODES`)

**Rust:**
- 函数: snake_case (如 `create_project`)
- Struct: PascalCase (如 `struct Project`)
- 模块: snake_case (如 `mod commands`)

## 添加新代码的位置

**新特性模块:**
- 组件: `src/components/features/<module-name>/`
- Store: `src/stores/` (如果需要新状态)
- API 函数: `src/lib/api.ts`
- Rust 命令: `src-tauri/src/commands/mod.rs`
- 数据库操作: `src-tauri/src/db/mod.rs`
- 数据模型: `src-tauri/src/models/mod.rs`

**新 UI 组件:**
- 组件文件: `src/components/ui/<ComponentName>.vue`
- 测试文件: `src/components/ui/<ComponentName>.test.ts`

**新数据库实体:**
1. 在 `src-tauri/src/models/mod.rs` 添加 struct
2. 在 `src-tauri/src/db/mod.rs` 添加表初始化和 CRUD 方法
3. 在 `src-tauri/src/commands/mod.rs` 添加 Tauri 命令
4. 在 `src/lib/api.ts` 添加 TypeScript 接口和 API 函数

**新 API 端点:**
1. Rust 命令: `src-tauri/src/commands/mod.rs`
2. 前端 API: `src/lib/api.ts`
3. Store 方法: 相应的 store 文件

## 特殊目录

**src/components/ui/ - 可复用 UI 组件库**
- 目的: 封装通用 UI 元素
- 包含: Button, Input, Modal, Select, Icon
- 特性: 类型 Props、响应式设计

**src/components/features/ - 业务功能组件**
- 目的: 按功能模块组织业务代码
- 每个子目录代表一个主要功能
- 包含: 页面级组件、业务逻辑

**src-tauri/src/db/ - 数据库操作**
- 目的: 隔离所有 SQLite 操作
- 包含: 表初始化、CRUD 方法
- 模式: Database struct 封装

**src-tauri/src/commands/ - Tauri 命令**
- 目的: 定义前端可调用的命令
- 模式: #[tauri::command] 宏

---

*结构分析: 2026-03-17*
