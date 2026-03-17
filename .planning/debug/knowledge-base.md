# GSD Debug Knowledge Base

Resolved debug sessions. Used by `gsd-debugger` to surface known-pattern hypotheses at the start of new investigations.

---

## card-theme-not-changing — 卡片主题切换不生效
- **Date:** 2026-03-17
- **Error patterns:** 卡片样式, 主题切换, 硬编码颜色, CSS变量, bg-[#0f0f14], text-zinc-100
- **Root cause:** 卡片组件使用硬编码的深色主题颜色（bg-[#0f0f14], text-zinc-100, border-white/8 等），而不是 CSS 变量（var(--bg-card), var(--text-primary), var(--border-default)）
- **Fix:** 将所有卡片组件中的硬编码颜色替换为 CSS 变量
- **Files changed:** Dashboard.vue, ProjectList.vue, DocumentCenter.vue, RecycleBin.vue, TaskBoard.vue
