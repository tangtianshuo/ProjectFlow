# Requirements: ProjectFlow

**Defined:** 2026-03-17
**Core Value:** 提供清晰、高效的项目管理体验，通过统一的 UI 设计系统提升用户体验

## v1 Requirements

### 暗色系 UI

- [x] **DARK-01**: 优化深色背景中的图标颜色，使用浅色图标提高对比度
- [x] **DARK-02**: 优化深色背景中的文字颜色，使用浅色文字确保清晰可见
- [x] **DARK-03**: 暗色系整体视觉协调统一

### 亮色系 UI

- [x] **LIGHT-01**: 实现亮色系主题（白色/浅灰色背景）
- [x] **LIGHT-02**: 亮色系使用柔和、不刺眼的图标颜色
- [x] **LIGHT-03**: 两套主题视觉风格逻辑一致
- [x] **LIGHT-04**: 支持暗色/亮色主题切换功能

### 优先级标签

- [x] **PRIO-01**: 优先级高/最高显示为红色标签
- [x] **PRIO-02**: 优先级中显示为黄色/橙色标签
- [x] **PRIO-03**: 优先级低/最低显示为蓝色/青色标签

### 进度条

- [x] **PROG-01**: 进度条使用蓝色渐变效果
- [x] **PROG-02**: 渐变效果能体现进度变化

### UI 设计规范

- [x] **DOC-01**: 创建暗色系设计规范文档（颜色、间距、字体等）
- [x] **DOC-02**: 创建亮色系设计规范文档（与暗色系逻辑一致）
- [x] **DOC-03**: 标准化组件样式（Button、Input、Select、Tag 等）

## v2 Requirements

### LLM 文档助手

- [ ] **LLM-01**: LLM panel UI in sidebar（侧边栏 AI 助手面板）
- [ ] **LLM-02**: OpenAI API integration（OpenAI API 集成）
- [ ] **LLM-03**: Encrypted API key storage（API Key 加密存储）
- [ ] **LLM-04**: Project context auto-injection（项目上下文自动注入）
- [ ] **LLM-05**: Streaming response UI（流式响应 UI）
- [ ] **LLM-06**: Conversation history（对话历史持久化）

## Out of Scope

| Feature | Reason |
|---------|--------|
| 动态主题色 | 聚焦于暗色/亮色两套固定主题 |
| 自定义主题 | 暂不需要用户自定义颜色 |
| 代码辅助功能 | 后续阶段 |
| 本地模型支持（Ollama） | 后续阶段 |
| 更多 LLM 模型 | 后续扩展 |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| DARK-01 | Phase 1 | Complete |
| DARK-02 | Phase 1 | Complete |
| DARK-03 | Phase 1 | Complete |
| LIGHT-01 | Phase 2 | Complete |
| LIGHT-02 | Phase 2 | Complete |
| LIGHT-03 | Phase 2 | Complete |
| LIGHT-04 | Phase 2 | Complete |
| PRIO-01 | Phase 1 | Complete |
| PRIO-02 | Phase 1 | Complete |
| PRIO-03 | Phase 1 | Complete |
| PROG-01 | Phase 1 | Complete |
| PROG-02 | Phase 1 | Complete |
| DOC-01 | Phase 1 | Complete |
| DOC-02 | Phase 2 | Complete |
| DOC-03 | Phase 1 | Complete |
| LLM-01 | Phase 3 | Pending |
| LLM-02 | Phase 3 | Pending |
| LLM-03 | Phase 3 | Pending |
| LLM-04 | Phase 3 | Pending |
| LLM-05 | Phase 3 | Pending |
| LLM-06 | Phase 3 | Pending |

**Coverage:**
- v1 requirements: 15 total
- v2 requirements: 6 total
- Mapped to phases: 21
- Unmapped: 0

---

*Requirements defined: 2026-03-17*
*Last updated: 2026-03-18*
