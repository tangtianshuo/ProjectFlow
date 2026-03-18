# Phase 3: LLM 文档助手 - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

实现 LLM AI 助手功能，集成到 ProjectFlow 桌面应用中。通过对话方式协助用户完成项目文档编写、项目问答、文档分析与建议。使用 LangChain Rust 框架，支持多模型切换。

**本阶段范围：**
- AI 助手 UI（侧边栏展开式面板）
- LLM 集成层（LangChain Rust + OpenAI GPT）
- 文档问答与生成功能
- API Key 本地加密存储
- 项目上下文自动注入

**后续阶段可扩展：**
- 更多 LLM 模型支持
- 代码辅助功能
- 本地模型支持

</domain>

<decisions>
## Implementation Decisions

### LLM 集成方式
- 使用 **LangChain Rust** 框架
- 初始支持 **OpenAI GPT** 模型
- 后续可扩展 Anthropic Claude、其他模型

### UI 布局
- 左侧边栏独立菜单项
- 展开式侧边栏面板（非独立页面）
- 类似 ChatGPT 的对话界面

### 核心功能
- **项目问答**：基于项目上下文回答问题
- **文档生成/优化**：AI 协助编写或改进 Markdown 文档
- **文档分析**：分析现有文档并提供改进建议

### API Key 安全
- 本地加密文件存储
- 用户可配置 API Key
- 支持多个模型切换

### 项目上下文
- **自动注入**：自动读取项目名称、描述、任务列表等作为上下文
- 减少用户手动操作

### Claude's Discretion
- 对话 UI 细节设计
- 具体加密算法选择
- 错误处理逻辑
- 对话历史存储策略

</decisions>

<specifics>
## Specific Ideas

- 类似 ChatGPT 的对话体验
- 侧边栏展开/收起动画
- 项目信息自动作为 system prompt 注入

</specifics>

<canonical_refs>
## Canonical References

### Project
- `.planning/PROJECT.md` — 项目定义，v2.0 LLM 文档助手目标
- `.planning/STATE.md` — 当前进度状态
- `.planning/ROADMAP.md` — 阶段规划

### Existing Code
- `src/features/documents/` — 文档功能现有实现
- `src/components/ui/` — 现有 UI 组件库
- `src-tauri/src/commands/` — Tauri 命令层
- `src-tauri/src/db/` — 数据库操作

### 技术参考
- LangChain Rust 官方文档
- Tauri secure storage 插件文档

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/components/ui/` — 现有 Button, Input, Modal, Icon 等组件
- `src/stores/` — Pinia 状态管理
- 文档编辑器组件（Monaco Editor）

### Established Patterns
- Vue 3 + TypeScript + Pinia
- Tauri 命令调用方式
- CSS 变量主题系统

### Integration Points
- 侧边栏组件需与现有 Sidebar 集成
- 新增 Tauri 命令处理 LLM API 调用
- 文档功能可复用现有 documentStore

</code_context>

<deferred>
## Deferred Ideas

- 代码辅助功能 — 后续阶段
- 本地模型支持（Ollama）— 后续阶段
- 更多 LLM 模型 — 后续扩展

</deferred>

---

*Phase: 03-llm*
*Context gathered: 2026-03-18*
