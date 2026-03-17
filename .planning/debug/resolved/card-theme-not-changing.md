---
status: resolved
trigger: "卡片样式在主题切换时没有变化"
created: 2026-03-17T00:00:00.000Z
updated: 2026-03-17T12:30:00.000Z
resolved: 2026-03-17T12:30:00.000Z
---

## Current Focus

hypothesis: 卡片组件未使用 CSS 变量，硬编码了深色主题颜色
test: 检查卡片组件是否使用 CSS 变量 (var(--bg-card), var(--text-primary))
expecting: 如果使用 CSS 变量，主题切换应该生效；如果硬编码颜色，则需要修改

**Root Cause Found:** 卡片组件使用硬编码的深色主题颜色值，而不是 CSS 变量。

**Fix Applied:** 已将所有卡片组件中的硬编码颜色替换为 CSS 变量。

**Verification:** 用户已确认修复完成

next_action: 归档调试会话

## Symptoms

expected: 点击切换主题后，卡片背景和文字颜色应该变化（浅色主题下卡片变白/浅灰，文字变深色）
actual: 完全没变化 - 卡片保持原来的深色样式
errors: []
reproduction: 点击 Sidebar 中的主题切换按钮后，卡片没有响应
started: 从一开始就不工作（今天刚添加的主题切换功能）

## Eliminated

## Evidence

- timestamp: 2026-03-17
  checked: style.css CSS 变量定义
  found: CSS 变量已正确定义：--bg-card (深色: rgba(22,22,32,0.6), 浅色: rgba(255,255,255,0.8)), --text-primary, --text-secondary, --border-default 等
  implication: CSS 变量已存在，组件应该使用它们

- timestamp: 2026-03-17
  checked: Dashboard.vue, ProjectList.vue, DocumentCenter.vue, RecycleBin.vue
  found: 卡片组件使用硬编码颜色如 bg-[#0f0f14]/80, text-zinc-100, border-white/8 等
  implication: 这些硬编码值不响应 [data-theme="light"] 切换

- timestamp: 2026-03-17
  checked: Sidebar.vue, Button.vue, Input.vue, Modal.vue, Select.vue
  found: 这些组件已正确使用 CSS 变量如 bg-[var(--bg-card)], text-[var(--text-primary)]
  implication: 模式已确立，只是卡片组件未更新

- timestamp: 2026-03-17
  checked: TaskBoard.vue
  found: Also had hardcoded colors - fixed
  implication: Also needs theme support

- timestamp: 2026-03-17
  checked: All feature components after fix
  found: All hardcoded colors (#0f0f14, #08080c, #12121a) removed
  implication: Components now use CSS variables

## Resolution

root_cause: 卡片组件使用硬编码的深色主题颜色（bg-[#0f0f14], text-zinc-100, border-white/8 等），而不是 CSS 变量（var(--bg-card), var(--text-primary), var(--border-default)）
fix: 将所有卡片组件中的硬编码颜色替换为 CSS 变量
verification: 用户手动验证通过
files_changed:
- src/components/features/dashboard/Dashboard.vue
- src/components/features/projects/ProjectList.vue
- src/components/features/documents/DocumentCenter.vue
- src/components/features/recycleBin/RecycleBin.vue
- src/components/features/tasks/TaskBoard.vue
