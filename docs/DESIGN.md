# ProjectFlow 项目管理系统设计文档

## 1. 系统概述

**ProjectFlow** 是一个基于 Tauri 框架开发的桌面应用程序，用于项目管理、任务管理和文档管理。

### 1.1 技术栈
- **前端**: Vue 3 + TypeScript + TailwindCSS
- **后端**: Rust + SQLite
- **桌面框架**: Tauri 2.x

---

## 2. 系统架构

### 2.1 前端架构 (Vue 3)

```
src/
├── App.vue                     # 主应用入口
├── main.ts                     # 应用入口
├── components/
│   ├── layout/
│   │   └── Sidebar.vue         # 侧边栏导航
│   ├── features/
│   │   ├── dashboard/
│   │   │   └── Dashboard.vue   # 仪表盘
│   │   ├── projects/
│   │   │   └── ProjectList.vue # 项目列表
│   │   ├── tasks/
│   │   │   └── TaskBoard.vue   # 任务看板
│   │   └── documents/
│   │       └── DocumentCenter.vue  # 文档中心
│   └── ui/
│       ├── Button.vue          # 按钮组件
│       ├── Input.vue           # 输入框组件
│       ├── Modal.vue           # 模态框组件
│       └── Select.vue          # 选择器组件
├── stores/
│   ├── projectStore.ts         # 项目和任务状态管理
│   ├── documentStore.ts        # 文档状态管理
│   └── uiStore.ts              # UI状态管理
└── lib/
    └── api.ts                  # API调用封装
```

### 2.2 后端架构 (Rust)

```
src-tauri/src/
├── main.rs                     # 应用入口
├── lib.rs                      # Tauri应用配置
├── models/
│   └── mod.rs                  # 数据模型定义
├── commands/
│   └── mod.rs                  # Tauri命令处理
└── db/
    └── mod.rs                  # SQLite数据库操作
```

---

## 3. 功能模块

### 3.1 仪表盘 (Dashboard)
- 显示项目统计数据：总项目数、进行中项目数、总任务数、已完成任务数
- 最近项目列表
- 数据按所有项目的实际情况统计

### 3.2 项目管理 (Project Management)
- 创建新项目（名称、描述、开始日期、结束日期）
- 查看项目列表
- 启动项目（状态 0 → 1）
- 完成项目（状态 1 → 2）
- 重新开启已完成项目（状态 2 → 1）
- 删除项目
- 根据任务完成情况自动计算项目进度

**项目状态定义**:
| 状态码 | 名称 | 说明 |
|--------|------|------|
| 0 | 未开始 | 项目尚未开始 |
| 1 | 进行中 | 项目正在执行 |
| 2 | 已完成 | 项目已完成 |
| 3 | 已归档 | 项目已归档 |

### 3.3 任务管理 (Task Management)
- 创建任务（标题、描述、优先级、截止日期）
- 查看任务看板（按状态分类）
- 快速切换任务状态
- 编辑任务
- 删除任务

**任务工作流**:
| 状态码 | 名称 | 进度默认值 |
|--------|------|------------|
| 0 | 待办 | 0% |
| 1 | 进行中 | 25% |
| 2 | 审核中 | 75% |
| 3 | 已完成 | 100% |

**优先级定义**:
| 优先级码 | 名称 |
|----------|------|
| 0 | 最低 |
| 1 | 低 |
| 2 | 中 |
| 3 | 高 |
| 4 | 最高 |

### 3.4 文档管理 (Document Center)
- 创建文档（标题、关联项目、内容）
- 编辑文档（Markdown支持）
- 查看文档列表
- 删除文档

---

## 4. 数据库设计

### 4.1 表结构

#### projects (项目表)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT PRIMARY KEY | 项目ID |
| name | TEXT NOT NULL | 项目名称 |
| description | TEXT | 项目描述 |
| status | INTEGER DEFAULT 0 | 项目状态 |
| start_date | TEXT | 开始日期 |
| end_date | TEXT | 结束日期 |
| owner_id | TEXT | 所有者ID |
| settings | TEXT | 设置(JSON) |
| created_at | TEXT NOT NULL | 创建时间 |
| updated_at | TEXT NOT NULL | 更新时间 |

#### tasks (任务表)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT PRIMARY KEY | 任务ID |
| project_id | TEXT NOT NULL | 所属项目ID |
| parent_id | TEXT | 父任务ID |
| title | TEXT NOT NULL | 任务标题 |
| description | TEXT | 任务描述 |
| status | INTEGER DEFAULT 0 | 任务状态 |
| priority | INTEGER DEFAULT 1 | 优先级 |
| assignee_id | TEXT | 负责人ID |
| start_date | TEXT | 开始日期 |
| due_date | TEXT | 截止日期 |
| estimated_hours | REAL | 预估工时 |
| actual_hours | REAL DEFAULT 0 | 实际工时 |
| progress | REAL DEFAULT 0 | 进度(0-100) |
| position | INTEGER DEFAULT 0 | 排序位置 |
| created_at | TEXT NOT NULL | 创建时间 |
| updated_at | TEXT NOT NULL | 更新时间 |

#### documents (文档表)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT PRIMARY KEY | 文档ID |
| project_id | TEXT | 关联项目ID |
| title | TEXT NOT NULL | 文档标题 |
| content | TEXT | 文档内容(Markdown) |
| file_path | TEXT | 文件路径 |
| current_version | INTEGER DEFAULT 1 | 当前版本 |
| created_at | TEXT NOT NULL | 创建时间 |
| updated_at | TEXT NOT NULL | 更新时间 |

#### users (用户表)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT PRIMARY KEY | 用户ID |
| name | TEXT NOT NULL | 用户名称 |
| email | TEXT | 邮箱 |
| avatar | TEXT | 头像URL |

#### task_dependencies (任务依赖表)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT PRIMARY KEY | 依赖ID |
| task_id | TEXT NOT NULL | 任务ID |
| depends_on_task_id | TEXT NOT NULL | 依赖的任务ID |
| dependency_type | INTEGER DEFAULT 0 | 依赖类型 |
| lag_days | INTEGER DEFAULT 0 | 滞后天数 |

---

## 5. API 接口

### 5.1 项目接口
- `create_project` - 创建项目
- `get_all_projects` - 获取所有项目
- `get_project` - 获取单个项目
- `update_project` - 更新项目
- `delete_project` - 删除项目

### 5.2 任务接口
- `create_task` - 创建任务
- `get_tasks_by_project` - 获取项目任务
- `update_task` - 更新任务
- `delete_task` - 删除任务

### 5.3 文档接口
- `create_document` - 创建文档
- `get_documents_by_project` - 获取项目文档
- `get_all_documents` - 获取所有文档
- `update_document` - 更新文档
- `delete_document` - 删除文档

---

## 6. 用户界面

### 6.1 布局
- 左侧固定侧边栏导航
- 主内容区域根据导航显示不同模块

### 6.2 导航项
1. 仪表盘 (Dashboard)
2. 项目 (Projects)
3. 任务 (Tasks) - 需要先选择项目
4. 文档 (Documents)
5. 设置 (Settings) - 开发中

---

## 7. 已知问题修复记录

### 7.1 任务状态更新报错
- **问题**: Rust 后端 `update_task` 函数参数索引错误
- **修复**: 添加 `param_idx += 1;` 确保参数索引正确

### 7.2 项目完成后无法重新开启
- **问题**: 缺少已完成项目(状态2)重新开启(到状态1)的逻辑
- **修复**: 在 `toggleProjectStatus` 中添加状态2→1的处理

### 7.3 仪表盘数据统计不准确
- **问题**: 只统计当前项目的任务，而非所有项目
- **修复**: 遍历所有项目获取任务数据进行统计
