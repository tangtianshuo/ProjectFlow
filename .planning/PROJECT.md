# ProjectFlow

## What This Is

Windows 桌面端项目管理应用，提供项目、任务、文档管理功能，支持看板视图和甘特图，适用于个人或团队的任务管理需求。

## Core Value

提供清晰、高效的项目管理体验，通过统一的 UI 设计系统提升用户体验。

## Requirements

### Validated

- ✓ 项目管理（创建、编辑、删除、归档项目）
- ✓ 任务看板（看板视图、甘特图）
- ✓ 任务管理（创建、编辑、删除、优先级、进度）
- ✓ 文档中心（Markdown 编辑器）
- ✓ 仪表盘（统计概览）
- ✓ 回收站（软删除）
- ✓ 暗色系 UI 优化（浅色图标/文字）
- ✓ 亮色系 UI 设计（柔和图标/文字）
- ✓ 优先级标签颜色规范化（高=红，中=黄，低=蓝）
- ✓ 进度条蓝色渐变效果
- ✓ 固定 UI 设计规范文档

### Active

- [ ] LLM 文档助手功能

### Out of Scope

- [新功能模块] — 当前聚焦于 UI 优化，现有功能已满足基本需求

## Context

**技术环境：**
- 前端：Vue 3 + TypeScript + Pinia + Tailwind CSS
- 后端：Rust + Tauri v2 + SQLite
- 现有组件：Button, Input, Select, Modal, Icon 等基础 UI 组件

**UX 问题待解决：**
- 深色背景中图标和文字颜色过深，不清晰
- 进度条和标签颜色区分不明显
- 缺少亮色系 UI
- UI 风格未固定，后续开发风格不一致

## Constraints

- **[技术限制]** 使用 Tailwind CSS 进行样式管理
- **[已有组件]** 必须复用现有 UI 组件体系进行优化

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 暗色系图标/文字 | 深色背景需要浅色对比度，提高可读性 | — Pending |
| 亮色系图标柔和处理 | 亮色背景使用柔和颜色，避免刺眼，协调视觉 | — Pending |
| 优先级颜色方案 | 高=红（警示）、中=黄（注意）、低=蓝（平静） | — Pending |
| 进度条蓝色渐变 | 蓝色渐变美观且体现进度变化 | — Pending |
| 固定 UI 设计规范 | 统一设计语言，避免后续开发风格漂移 | — Pending |

---

## Current Milestone: v2.0 LLM 文档助手

**Goal:** 通过对话方式实现项目文档编写，集成多模型 LLM 支持

**Target features:**
- 多模型支持（LangChain Rust）
- AI 助手独立菜单
- 文档生成
- 项目问答
- 文档分析与建议
- API Key 加密存储
- 项目上下文自动注入

---

*Last updated: 2026-03-17 after v2.0 milestone started*
