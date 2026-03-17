# Phase 1: Dark UI Optimization - Research

**Researched:** 2026-03-17
**Domain:** UI/UX Design, Vue 3 + Tailwind CSS Dark Theme Optimization
**Confidence:** HIGH

## Summary

This phase focuses on optimizing the existing dark theme UI for ProjectFlow, a Tauri v2 + Vue 3 desktop application. The research confirms the existing codebase has a solid foundation with CSS variables and UI components, but needs specific improvements to meet the requirements. Priority tag colors need correction to match the spec (red/yellow/cyan instead of current gray/amber/red), and progress bars need gradient implementations. The Icon component already exists and uses Lucide Vue, requiring only color prop integration.

**Primary recommendation:** Update CSS variables and component styles to match UI-SPEC.md color palette, implement priority tag color changes in TaskBoard.vue, and add gradient progress bar components.

---

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DARK-01 | 优化深色背景中的图标颜色 | Existing Icon.vue supports class prop; need to apply --text-secondary (#a1a1aa) as default |
| DARK-02 | 优化深色背景中的文字颜色 | CSS variables already defined in style.css (--text-primary: #f4f4f5, --text-secondary: #a1a1aa) |
| DARK-03 | 暗色系整体视觉协调统一 | Need to standardize all components to use CSS variables |
| PRIO-01 | 优先级高/最高显示为红色标签 | TaskBoard.vue getPriorityColor() needs update: priority 3-4 -> #f87171 |
| PRIO-02 | 优先级中显示为黄色/橙色标签 | TaskBoard.vue getPriorityColor() needs update: priority 2 -> #fbbf24 |
| PRIO-03 | 优先级低/最低显示为蓝色/青色标签 | TaskBoard.vue getPriorityColor() needs update: priority 0-1 -> #22d3ee |
| PROG-01 | 进度条使用蓝色渐变效果 | Need to implement gradient in TaskBoard.vue getProgressColor() |
| PROG-02 | 渐变效果能体现进度变化 | CSS gradient animation technique identified |
| DOC-01 | 创建暗色系设计规范文档 | UI-SPEC.md exists but needs enhancement with component usage |
| DOC-03 | 标准化组件样式 | Button, Input, Select, Modal need CSS variable alignment |

---

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Vue 3 | ^3.4 | Frontend framework | Project baseline |
| Tailwind CSS | ^3.4 | Utility-first CSS | Already integrated in project |
| Lucide Vue | Latest | Icon library | Already installed (Icon.vue component) |
| Inter Font | System/Google | Typography | Already defined in style.css |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| CSS Variables | N/A | Theme tokens | All color/spacing definitions |
| CSS Gradients | N/A | Progress bar effects | PROG-01, PROG-02 |

---

## Architecture Patterns

### Recommended Project Structure
```
src/
├── components/
│   └── ui/
│       ├── Button.vue       # DARK-01, DARK-02, DOC-03
│       ├── Input.vue        # DARK-01, DARK-02, DOC-03
│       ├── Select.vue       # DARK-01, DARK-02, DOC-03
│       ├── Modal.vue        # DARK-01, DARK-02, DOC-03
│       ├── Tag.vue          # DOC-03 (NEW - priority tags)
│       └── ProgressBar.vue  # PROG-01, PROG-02 (NEW)
├── style.css                # DARK-02, DOC-01 (design tokens)
└── features/
    └── tasks/
        └── TaskBoard.vue    # PRIO-01, PRIO-02, PRIO-03, PROG-01
```

### Pattern 1: CSS Variable-Based Theme
**What:** Use CSS custom properties for all colors, ensuring consistency
**When to use:** All component styling
**Example:**
```css
/* Already defined in style.css */
:root {
  --text-primary: #f4f4f5;
  --text-secondary: #a1a1aa;
  --accent-primary: #6366f1;
  --accent-red: #f87171;
  --accent-cyan: #22d3ee;
}
```
**Source:** D:\Projects\Tauri\ProjectFlow\src\style.css

### Pattern 2: Priority Tag Component
**What:** Centralized priority color logic
**When to use:** Displaying task priority levels
**Example:**
```typescript
function getPriorityColor(priority: number) {
  const colors = [
    "bg-cyan-500/15 text-cyan-400",     // priority 0: lowest
    "bg-cyan-500/15 text-cyan-400",     // priority 1: low
    "bg-amber-500/15 text-amber-400",   // priority 2: medium
    "bg-red-500/15 text-red-400",       // priority 3: high
    "bg-red-500/15 text-red-400",       // priority 4: highest
  ];
  return colors[priority] || colors[2];
}
```
**Source:** Derived from UI-SPEC.md (PRIO-01, PRIO-02, PRIO-03)

### Pattern 3: Gradient Progress Bar
**What:** Animated gradient fill for progress indication
**When to use:** Task progress visualization
**Example:**
```vue
<template>
  <div class="h-2 bg-white/10 rounded-full overflow-hidden">
    <div
      class="h-full rounded-full transition-all duration-500"
      :style="progressGradient"
    />
  </div>
</template>

<script setup>
const progressGradient = computed(() => {
  const percentage = props.progress;
  return {
    width: `${percentage}%`,
    background: `linear-gradient(90deg, #6366f1 ${100 - percentage}%, #22d3ee 100%)`
  };
});
</script>
```
**Source:** UI-SPEC.md (PROG-01, PROG-02)

### Pattern 4: Icon Color Classes
**What:** Apply color via class prop to Lucide icons
**When to use:** All icon usage throughout app
**Example:**
```vue
<!-- Default (secondary text color) -->
<Icon name="folder-kanban" />

<!-- With explicit color -->
<Icon name="settings" class="text-zinc-400 hover:text-white" />

<!-- Using CSS variable -->
<Icon name="star" class="text-[var(--accent-primary)]" />
```
**Source:** Icon.vue component structure

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Icons | Custom SVG components | Lucide Vue (already integrated) | 400+ icons, consistent style, tree-shakeable |
| Color system | Hardcoded hex values | CSS variables (already defined) | Easy theming, consistency |
| Scrollbars | Custom scrollbar JS | CSS ::-webkit-scrollbar | Native, performant |

**Key insight:** ProjectFlow already has good foundations - this phase is about refinement, not rebuilding.

---

## Common Pitfalls

### Pitfall 1: Hardcoded Colors Instead of CSS Variables
**What goes wrong:** Inconsistent colors across components, difficult to maintain
**Why it happens:** Developers using Tailwind's default colors (zinc-500, red-400) instead of CSS variables
**How to avoid:** Use `text-[var(--text-secondary)]` or custom Tailwind colors mapped to variables
**Warning signs:** Mix of `text-zinc-400` and `text-[#a1a1aa]` in same component

### Pitfall 2: Priority Color Mismatch with Spec
**What goes wrong:** Current TaskBoard.vue uses gray for low, amber for medium, red for high
**Why it happens:** getPriorityColor() function predates UI-SPEC.md
**How to avoid:** Update function to match UI-SPEC.md: cyan (low), amber (medium), red (high)
**Warning signs:** Priority tags don't match REQUIREMENTS.md color definitions

### Pitfall 3: Missing Gradient Animation on Progress
**What goes wrong:** Static gradient doesn't animate with progress changes
**Why it happens:** CSS gradient cannot be directly transitioned with width
**How to avoid:** Use percentage-based gradient stops that shift with progress value
**Warning signs:** Progress bar has gradient but doesn't feel dynamic

### Pitfall 4: Icon Color Not Inheriting from Text Colors
**What goes wrong:** Icons appear too dark on dark backgrounds
**Why it happens:** Default stroke color inherited from parent, often too dark
**How to avoid:** Explicitly set icon class: `class="text-[var(--text-secondary)]"`
**Warning signs:** Icons hard to read in sidebar or cards

---

## Code Examples

### Example 1: Updated Priority Color Function
```typescript
// Source: UI-SPEC.md + existing TaskBoard.vue
function getPriorityColor(priority: number) {
  const colors = [
    "bg-cyan-500/15 text-cyan-400",     // Lowest (priority 0)
    "bg-cyan-500/15 text-cyan-400",     // Low (priority 1)
    "bg-amber-500/15 text-amber-400",   // Medium (priority 2)
    "bg-red-500/15 text-red-400",       // High (priority 3)
    "bg-red-500/15 text-red-400",       // Highest (priority 4)
  ];
  return colors[priority] || colors[2];
}
```

### Example 2: Gradient Progress Bar with Animation
```vue
<!-- Source: Derived from UI-SPEC.md PROG-01, PROG-02 -->
<template>
  <div class="flex items-center gap-3">
    <div class="flex-1 h-2 bg-white/10 rounded-full overflow-hidden">
      <div
        class="h-full rounded-full transition-all duration-500 ease-out"
        :style="{ width: `${progress}%`, background: gradientStyle }"
      />
    </div>
    <span class="text-xs text-[var(--text-secondary)] min-w-[36px] text-right">
      {{ progress }}%
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  progress: number;
}>();

const gradientStyle = computed(() => {
  // Gradient shifts from cyan (0%) to primary (100%)
  const p = props.progress;
  return `linear-gradient(90deg,
    var(--accent-primary) 0%,
    var(--accent-primary) ${p}%,
    var(--accent-cyan) ${p}%,
    var(--accent-cyan) 100%)`;
});
</script>
```

### Example 3: Icon with Proper Dark Theme Colors
```vue
<!-- Default usage - uses secondary text color -->
<Icon name="folder-kanban" />

<!-- Explicit dark theme colors -->
<Icon
  name="settings"
  class="text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors"
/>

<!-- Active/selected state -->
<Icon
  name="star"
  class="text-[var(--accent-primary)]"
/>
```

### Example 4: Standardized Button Using CSS Variables
```vue
<!-- Updated Button.vue variant to use CSS variables -->
<script>
const variants = {
  primary: "bg-[var(--accent-primary)] text-white hover:bg-[var(--accent-primary-hover)]",
  secondary: "bg-[var(--bg-secondary)] text-[var(--text-secondary)] border border-[var(--border-default)] hover:bg-[var(--bg-tertiary)]",
  danger: "bg-red-500/15 text-[var(--accent-red)] border border-red-500/25 hover:bg-red-500/25",
  // ...
};
</script>
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Hardcoded zinc colors | CSS variables | Existing style.css | Better maintainability |
| Solid progress colors | Gradient effect | This phase | Visual enhancement (PROG-01) |
| Gray priority tags | Cyan/amber/red tags | This phase | Meets REQUIREMENTS.md |
| Direct icon stroke color | class-based coloring | This phase | Better visibility (DARK-01) |

**Deprecated/outdated:**
- `text-zinc-400` / `text-zinc-500`: Should migrate to `text-[var(--text-secondary)]`
- `bg-zinc-500/15 text-zinc-400`: Priority low colors should be cyan

---

## Open Questions

1. **Should Tag component be extracted from TaskBoard.vue?**
   - What we know: Priority tags currently inline in TaskBoard.vue
   - What's unclear: Whether to create reusable Tag.vue component for DOC-03
   - Recommendation: Create Tag.vue with priority prop for standardization

2. **Should ProgressBar be a separate component?**
   - What we know: Progress bar logic inline in TaskBoard.vue
   - What's unclear: Reuse across ProjectList, Dashboard, etc.
   - Recommendation: Create ProgressBar.vue component for consistency

3. **How to handle icon color in Icon.vue default?**
   - What we know: Icon.vue accepts class prop but has no default color
   - What's unclear: Best default that doesn't break existing usage
   - Recommendation: Apply default class="text-[var(--text-secondary)]" via prop default

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Vitest (existing in project) |
| Config file | vitest.config.ts |
| Quick run command | `npm run test` |
| Full suite command | `npm run test:coverage` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DARK-01 | Icons use light colors in dark theme | Visual | Manual verification | N/A |
| DARK-02 | Text uses light colors | Visual | Manual verification | N/A |
| DARK-03 | Visual consistency | Visual | Manual verification | N/A |
| PRIO-01 | High priority shows red | Unit | `npm run test` | NO - visual check |
| PRIO-02 | Medium priority shows amber | Unit | `npm run test` | NO - visual check |
| PRIO-03 | Low priority shows cyan | Unit | `npm run test` | NO - visual check |
| PROG-01 | Progress bar has gradient | Unit | `npm run test` | NO - visual check |
| PROG-02 | Gradient animates with progress | Unit | `npm run test` | NO - visual check |
| DOC-01 | Design spec documented | Manual | Review UI-SPEC.md | YES |
| DOC-03 | Components standardized | Manual | Code review | N/A |

### Wave 0 Gaps
- [ ] Test infrastructure: No visual regression tests exist
- [ ] Component tests: No unit tests for UI components
- [ ] Note: Visual verification is manual for this phase; consider screenshot testing if needed

**Validation note:** This phase is primarily visual/UI. Most requirements cannot be verified via automated tests. Manual code review against UI-SPEC.md colors is the verification approach.

---

## Sources

### Primary (HIGH confidence)
- UI-SPEC.md - Phase 1 design contract with exact color values
- REQUIREMENTS.md - DARK-01 through PROG-02 requirement definitions
- style.css - Existing CSS variables (already defined correctly)

### Secondary (MEDIUM confidence)
- TaskBoard.vue - Current implementation of priority/progress (needs updates)
- Icon.vue - Existing Icon component structure
- Button.vue, Input.vue, Select.vue - Existing UI components

### Tertiary (LOW confidence)
- Lucide Vue documentation - Icon library reference (Context7)
- Tailwind CSS docs - Utility classes reference

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Vue 3 + Tailwind + Lucide already in use
- Architecture: HIGH - Clear structure from existing codebase
- Pitfalls: HIGH - Identified from code review of current implementation

**Research date:** 2026-03-17
**Valid until:** 2026-04-17 (30 days - UI phase is stable)

---

## Implementation Action Items

1. Update TaskBoard.vue priority colors to match UI-SPEC.md
2. Implement gradient progress bar in TaskBoard.vue (or create ProgressBar component)
3. Standardize Button, Input, Select to use CSS variables
4. Add default color class to Icon.vue or update usages
5. Enhance UI-SPEC.md with component usage examples (DOC-01)
6. Create Tag.vue component if extracting from TaskBoard.vue (DOC-03)
