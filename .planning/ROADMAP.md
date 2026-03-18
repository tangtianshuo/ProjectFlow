# Roadmap: ProjectFlow

**Created:** 2026-03-17
**Core Value:** 提供清晰、高效的项目管理体验，通过统一的 UI 设计系统提升用户体验

## Phase 1: 暗色系 UI 优化

**Goal:** 优化现有暗色系 UI，提升图标、文字、标签、进度条的视觉效果

**Requirements:**
- DARK-01: 优化深色背景中的图标颜色
- DARK-02: 优化深色背景中的文字颜色
- DARK-03: 暗色系整体视觉协调统一
- PRIO-01: 优先级高/最高显示为红色标签
- PRIO-02: 优先级中显示为黄色/橙色标签
- PRIO-03: 优先级低/最低显示为蓝色/青色标签
- PROG-01: 进度条使用蓝色渐变效果
- PROG-02: 渐变效果能体现进度变化
- DOC-01: 创建暗色系设计规范文档
- DOC-03: 标准化组件样式

**Plans:**
2/2 plans complete

**Success Criteria:**
1. 深色背景中的图标使用浅色，清晰可见
2. 深色背景中的文字使用浅色，对比度良好
3. 优先级高=红色，中=黄/橙，低=蓝/青
4. 进度条使用蓝色渐变，能体现进度变化
5. 创建暗色系设计规范文档
6. 组件样式统一标准化

---

## Phase 2: 亮色系 UI 设计

**Goal:** 实现亮色系 UI 设计，与暗色系保持一致的视觉逻辑

**Requirements:**
- LIGHT-01: 实现亮色系主题（白色/浅灰色背景）
- LIGHT-02: 亮色系使用柔和、不刺眼的图标颜色
- LIGHT-03: 两套主题视觉风格逻辑一致
- LIGHT-04: 支持暗色/亮色主题切换功能
- DOC-02: 创建亮色系设计规范文档

**Plans:**
3/3 plans complete
- [x] 02-01-PLAN.md - Light theme CSS variables implementation
- [x] 02-02-PLAN.md - Theme toggle + design specification
- [x] 02-03-PLAN.md - Gap closure: App.vue hardcoded background fix

**Success Criteria:**
1. 亮色系主题正常显示，背景为白色/浅灰色
2. 亮色系图标柔和协调，不刺眼
3. 两套主题设计逻辑一致（颜色、间距、组件风格）
4. 用户可在应用内切换暗色/亮色主题
5. 创建亮色系设计规范文档

---

## Phase 3: LLM 文档助手

**Goal:** 实现 LLM AI 助手功能，集成到 ProjectFlow 桌面应用中。通过对话方式协助用户完成项目文档编写、项目问答、文档分析与建议。

**Requirements:**
- LLM-01: LLM panel UI in sidebar
- LLM-02: OpenAI API integration
- LLM-03: Encrypted API key storage
- LLM-04: Project context auto-injection
- LLM-05: Streaming response UI
- LLM-06: Conversation history

**Plans:**
3/3 plans complete
- [x] 03-01-PLAN.md — Backend LLM integration (Rust)
- [x] 03-02-PLAN.md — Frontend LLM UI (Vue)
- [x] 03-03-PLAN.md — Integration & wiring

**Success Criteria:**
1. 用户可以在侧边栏打开 AI 助手面板
2. 用户可以配置 OpenAI API Key（加密存储）
3. 用户可以发送消息并接收流式响应
4. 项目上下文自动注入到 prompts 中
5. 对话历史可持久化存储

---

## Phase 4: 适配国产LLM模型

**Goal:** 扩展 LLM 助手以支持国产大语言模型（Kimi、DeepSeek、MiniMax），用户可配置每个模型的 baseurl、api_key、model_name

**Requirements:**
- LLM-07: Support Chinese LLM models (Kimi, DeepSeek, MiniMax)
- LLM-08: Configurable base_url per model
- LLM-09: Model selection UI

**Plans:**
2/2 plans complete
- [x] 04-01-PLAN.md — Backend: Extend OpenAIClient with base_url, storage for model config
- [x] 04-02-PLAN.md — Frontend: Model selection UI and config inputs

**Success Criteria:**
1. 用户可以选择 Kimi、DeepSeek、MiniMax 模型
2. 用户可以配置每个模型的 base_url
3. 用户可以配置每个模型的 model_name
4. Chat 使用配置的 base_url 进行 API 调用

---

## Phase 5: 使用 litellm_rs 重构 LLM 接入层

**Goal:** 使用 litellm_rs 库替换现有的自定义 LLM 实现，提供更统一的 API 接口，支持更多模型供应商

**Requirements:**
- LLM-10: 使用 litellm_rs 作为 LLM API 接入层
- LLM-11: 统一模型配置管理
- LLM-12: 保持现有的 base_url 配置能力
- LLM-13: 简化代码结构，降低维护成本

**Plans:**
2/2 plans
- [ ] 05-01-PLAN.md — Add litellm_rs dependency and create LitellmClient module
- [ ] 05-02-PLAN.md — Migrate commands to use LitellmClient

**Success Criteria:**
1. 使用 litellm_rs 库进行 API 调用
2. 保留现有的 model 配置功能
3. 代码结构更清晰
4. 兼容现有 UI 功能

---

## Summary

| Phase | Name | Requirements | Plans | Status |
|-------|------|--------------|-------|--------|
| 1 | 暗色系 UI 优化 | Complete | 2/2 | Complete |
| 2 | 亮色系 UI 设计 | Complete | 3/3 | Complete |
| 3 | LLM 文档助手 | Complete | 3/3 | Complete |
| 4 | 适配国产LLM模型 | Complete | 2/2 | Complete |
| 5 | 使用 litellm_rs 重构 LLM 接入层 | 4 | 2/2 | Planned |

**Total:** 5 phases | 28 requirements | 12 plans

---

*Roadmap created: 2026-03-17*
*Plans updated: 2026-03-18*
