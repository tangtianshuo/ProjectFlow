# 代码库关注领域

**分析日期:** 2026-03-17

## 技术债务

### 前端 Store 状态同步问题

- **问题:** `projectStore` 和 `documentStore` 使用本地 ref 状态，缺少与后端的实时同步机制。当多个客户端或标签页同时操作时，可能出现数据不一致。
- **文件:** `src/stores/projectStore.ts`, `src/stores/documentStore.ts`
- **影响:** 多实例使用时可能丢失数据更新
- **修复方案:** 考虑添加乐观更新回退机制，或实现事件总线同步

### 文档编辑器自动保存触发频繁

- **问题:** `DocumentCenter.vue` 中使用 `@change` 事件触发保存，每次输入都会调用后端 API (`saveDocument`)，在 `DocumentCenter.vue` 第 210 行
- **文件:** `src/components/features/documents/DocumentCenter.vue`
- **影响:** 大量文档编辑时会产生大量 API 调用，增加服务器负载，可能导致性能问题
- **修复方案:** 实现防抖 (debounce) 或定时自动保存机制

### 使用 `any` 类型

- **问题:** Dashboard.vue 第 9 行使用 `any[]` 类型，削弱了 TypeScript 类型安全
- **文件:** `src/components/features/dashboard/Dashboard.vue`
- **影响:** 失去编译时类型检查，增加运行时错误风险
- **修复方案:** 定义 Task 接口或使用 `Task[]` 类型

### 前端错误处理不完善

- **问题:** 大量 `console.error` 输出，但缺少用户友好的错误提示 UI。部分地方错误只是简单地打印到控制台
- **文件:** `src/components/features/tasks/TaskBoard.vue`, `src/components/features/documents/DocumentCenter.vue`
- **影响:** 用户不知道操作失败的原因
- **修复方案:** 添加 toast/notification 组件展示错误信息

### 仪表盘性能问题

- **问题:** `Dashboard.vue` 中 `onMounted` 使用 for 循环逐个获取每个项目的任务，没有使用批量查询 (第 33-40 行)
- **文件:** `src/components/features/dashboard/Dashboard.vue`
- **影响:** 项目数量多时会产生 N+1 查询问题，加载缓慢
- **修复方案:** 添加后端批量获取所有任务或项目详情的 API

## 已知问题

### 日期显示格式不一致

- **问题:** 任务卡片中显示日期使用 `split("T")[0]` (第 300 行 TaskBoard.vue)，而后端存储的是 RFC3339 格式，前端需要手动解析
- **文件:** `src/components/features/tasks/TaskBoard.vue`
- **触发:** 任何显示任务日期的地方
- **修复方案:** 统一使用日期格式化工具函数

### GanttChart 组件返回空

- **问题:** `GanttChart.vue` 使用 `return {}` 返回空对象，可能导致调用方需要额外处理空值检查
- **文件:** `src/components/features/tasks/GanttChart.vue`
- **影响:** 组件可能无法正常渲染甘特图
- **修复方案:** 检查并实现完整的甘特图渲染逻辑

### 前端类型定义与后端不一致

- **问题:** `api.ts` 中定义的类型使用 snake_case (如 `startDate`)，而后端 Rust 模型使用 camelCase，可能存在映射错误
- **文件:** `src/lib/api.ts`
- **触发:** 某些字段在前后端之间传递时可能丢失或错误
- **修复方案:** 统一使用一种命名规范，建议全部使用 camelCase

## 安全性考虑

### 后端缺少输入验证

- **风险:** 所有 Tauri 命令直接接受前端传入的字符串参数，没有长度限制或格式验证
- **文件:** `src-tauri/src/commands/mod.rs`
- **当前缓解:** 使用 SQLite 参数化查询防止 SQL 注入
- **建议:**
  - 添加必填字段校验 (如项目名称不能为空)
  - 添加字符串长度限制
  - 验证日期格式
  - 验证数值范围 (如 status, priority 等枚举值)

### 用户 ID 未验证

- **风险:** `ownerId`、`assigneeId` 等字段可以任意设置，没有用户认证机制
- **文件:** `src-tauri/src/commands/mod.rs`, `src-tauri/src/db/mod.rs`
- **当前缓解:** 单用户桌面应用，风险较低
- **建议:** 考虑添加简单的用户系统或权限验证

### 敏感数据明文存储

- **风险:** 所有数据以明文存储在 SQLite 数据库中，没有加密
- **文件:** `src-tauri/src/db/mod.rs`
- **当前缓解:** 本地桌面应用，数据存储在用户本地
- **建议:** 如果后续支持多用户或云同步，需考虑数据加密

### 前端 XSS 风险

- **风险:** `DocumentCenter.vue` 中使用 `v-html` 渲染 Markdown 预览 (第 218 行)，虽然使用 `marked` 库，但未配置 HTML 过滤
- **文件:** `src/components/features/documents/DocumentCenter.vue`
- **建议:** 配置 `marked` 使用 sanitize 选项或使用 DOMPurify

## 性能瓶颈

### 数据库查询无分页

- **问题:** 所有查询 (get_all_projects, get_tasks_by_project 等) 一次性返回所有数据
- **文件:** `src-tauri/src/db/mod.rs`
- **影响:** 数据量增长时内存占用增加，界面渲染变慢
- **改进路径:** 添加分页支持，使用 LIMIT/OFFSET

### 缺少数据库连接池

- **问题:** 使用单一 SQLite 连接，所有请求串行化
- **文件:** `src-tauri/src/db/mod.rs` (第 19-21 行 Mutex<Connection>)
- **影响:** 高并发场景性能受限
- **改进路径:** 考虑使用 r2d2 连接池 (虽然 Tauri 是单用户应用)

### 前端渲染性能

- **问题:** 大量任务卡片同时渲染时没有使用虚拟滚动
- **文件:** `src/components/features/tasks/TaskBoard.vue`
- **影响:** 任务数量超过 100+ 时页面卡顿
- **改进路径:** 使用虚拟列表组件

### 重复计算

- **问题:** `projectProgress` computed 属性在每次调用时重新计算所有项目的进度
- **文件:** `src/stores/projectStore.ts` (第 21-33 行)
- **影响:** 项目和任务数量多时产生性能开销
- **改进路径:** 缓存进度计算结果，只在任务更新时重新计算

## 脆弱区域

### 任务依赖功能未实现

- **文件:** `src-tauri/src/db/mod.rs` (第 112-124 行创建了 task_dependencies 表)
- **为什么脆弱:** 表已创建但没有对应的 CRUD 命令，代码中可能出现 null 错误
- **安全修改:** 先实现完整的任务依赖 API 再使用此功能

### 里程碑 (Milestone) 功能不完整

- **文件:** `src/lib/api.ts` (第 96-120 行定义了类型但 API 不完整), `src-tauri/src/commands/mod.rs`
- **为什么脆弱:** 只有基本的 CRUD，缺少获取所有里程碑、删除恢复等功能
- **安全修改:** 补全里程碑的 API

### 用户表未使用

- **文件:** `src-tauri/src/db/mod.rs` (第 102-110 行创建了 users 表)
- **为什么脆弱:** 表存在但无任何操作接口，ownerId/assigneeId 等字段无法真正关联用户
- **安全修改:** 实现用户管理或移除未使用的表

### 软删除恢复后子任务处理

- **文件:** `src-tauri/src/db/mod.rs` (restore_project, restore_task 等方法)
- **为什么脆弱:** 恢复项目或任务时，没有处理关联的子任务/文档的恢复状态
- **安全修改:** 实现级联恢复逻辑

### 前端状态管理脆弱

- **问题:** 直接修改 ref 数组内容 (如 `tasks.value.push()`, `unshift()`) 而不是使用不可变方式更新
- **文件:** `src/stores/projectStore.ts`, `src/stores/documentStore.ts`
- **为什么容易出错:** Vue 3 reactivity 可能无法检测到某些数组操作变化，导致 UI 不更新
- **安全修改:** 使用 immutable 模式更新状态，或使用 Vue.set

## 扩展性限制

### 缺乏国际化 (i18n) 支持

- **当前状态:** 所有 UI 文本硬编码为中文
- **限制:** 难以扩展到其他语言市场
- **建议:** 引入 vue-i18n

### 配置管理缺失

- **当前状态:** 没有用户偏好设置、主题切换等功能
- **限制:** 功能单一
- **建议:** 添加设置存储和配置 UI

### 插件/扩展架构缺失

- **当前状态:** 功能硬编码
- **限制:** 难以添加自定义功能
- **建议:** 考虑模块化架构

## 测试覆盖差距

### 单元测试覆盖不足

- **未测试区域:**
  - 所有 Store (projectStore, documentStore 只有 UI store 有测试)
  - 所有 API 调用函数
  - 所有 Vue 组件 (只有 Button, Input 有测试)
- **风险:** 重构或添加新功能时可能破坏现有功能
- **优先级:** 高

### 缺少集成测试

- **未测试:**
  - 前后端 API 交互
  - 数据库操作
  - 软删除/恢复流程
- **优先级:** 中

### 缺少 E2E 测试

- **当前状态:** 没有 E2E 测试框架
- **优先级:** 低 (桌面应用，自动化测试难度较大)

---

*关注领域审计: 2026-03-17*
