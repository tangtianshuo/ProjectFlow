# Phase 2: 亮色系 UI 设计 - Research

**Researched:** 2026-03-17
**Domain:** Vue 3 + Tailwind CSS Theme Implementation
**Confidence:** HIGH

## Summary

Phase 2 requires implementing a light theme that mirrors the dark theme's visual logic. The UI SPEC provides detailed color tokens that are inversions of the dark theme. Key implementation involves: (1) adding `[data-theme="light"]` CSS variables to style.css, (2) extending the Pinia uiStore to manage theme state, (3) implementing a theme toggle in the Sidebar, and (4) ensuring all components reference CSS variables instead of hardcoded colors. The project already uses CSS variables for all colors, making this a straightforward extension.

**Primary recommendation:** Add light theme CSS variables using `[data-theme="light"]` selector, extend uiStore with theme management, and add sun/moon toggle button to Sidebar. All existing components will automatically adapt when CSS variables change.

## User Constraints (from CONTEXT.md)

### Locked Decisions
- Use existing CSS variable system (--bg-primary, --text-primary, etc.)
- Maintain 60/30/10 color distribution (dominant/secondary/accent)
- Keep spacing scale identical (4px multiples)
- Same typography (Inter 14px body)
- Same component styling logic between themes
- Use Lucide Vue for icons

### Claude's Discretion
- Theme toggle placement: Sidebar or Settings (recommend Sidebar for quick access)
- Transition timing: 200ms specified but can fine-tune
- System preference detection implementation details

### Deferred Ideas (OUT OF SCOPE)
- Dynamic theme colors (user-selectable)
- Multiple light themes
- Custom theme builder

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| LIGHT-01 | 实现亮色系主题（白色/浅灰色背景） | UI SPEC provides exact hex values: #fafafa primary, #ffffff secondary |
| LIGHT-02 | 亮色系使用柔和、不刺眼的图标颜色 | Icon colors: #52525b default, #18181b hover, #6366f1 active |
| LIGHT-03 | 两套主题视觉风格逻辑一致 | Consistency matrix shows inverted values, same structure |
| LIGHT-04 | 支持暗色/亮色主题切换功能 | uiStore extension + LocalStorage persistence + system preference |
| DOC-02 | 创建亮色系设计规范文档 | Document tokens in code comments per UI SPEC |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Vue 3 | latest | Frontend framework | Project foundation |
| Pinia | ^2.x | State management | Existing project pattern |
| Tailwind CSS | latest | Utility CSS | Already integrated in project |
| Lucide Vue | latest | Icon library | Already in use |

### Implementation Approach
The light theme implementation uses native CSS custom properties with a `[data-theme="light"]` selector. No additional libraries required.

**Installation:**
No new packages needed - uses existing CSS variable system.

## Architecture Patterns

### Recommended Structure

```
src/
├── style.css                    # Add [data-theme="light"] block
├── stores/
│   └── uiStore.ts               # Add theme state management
└── components/
    └── layout/
        └── Sidebar.vue          # Add theme toggle button
```

### Pattern 1: CSS Custom Properties Theme System
**What:** Use CSS variables with data-theme attribute selector for theming
**When to use:** When you need lightweight, performant theme switching without JavaScript overhead
**Example:**
```css
/* Source: Project style.css pattern */
:root {
  --bg-primary: #08080c;  /* Dark */
}

[data-theme="light"] {
  --bg-primary: #fafafa;  /* Light */
}

/* Components use: background: var(--bg-primary); */
```

### Pattern 2: Pinia Store Theme Management
**What:** Centralize theme state in Pinia store with LocalStorage persistence
**When to use:** When theme preference needs to persist across sessions
**Example:**
```typescript
// Extend existing uiStore.ts
const theme = ref<'dark' | 'light'>('dark');

function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark';
  document.documentElement.setAttribute('data-theme', theme.value);
  localStorage.setItem('projectflow-theme', theme.value);
}

// Initialize from localStorage or system preference
function initTheme() {
  const saved = localStorage.getItem('projectflow-theme');
  if (saved) {
    theme.value = saved as 'dark' | 'light';
  } else if (window.matchMedia('(prefers-color-scheme: light)').matches) {
    theme.value = 'light';
  }
  document.documentElement.setAttribute('data-theme', theme.value);
}
```

### Pattern 3: System Preference Detection
**What:** Detect OS color scheme preference using matchMedia
**When to use:** Default theme based on user system settings
**Example:**
```typescript
const prefersDark = window.matchMedia('(prefers-color-scheme: dark)');
prefersDark.addEventListener('change', (e) => {
  if (!localStorage.getItem('projectflow-theme')) {
    setTheme(e.matches ? 'dark' : 'light');
  }
});
```

### Anti-Patterns to Avoid

- **Hardcoded colors in components:** Don't use #ffffff or #000000 directly - always use CSS variables
- **Theme toggle without persistence:** Theme will reset on page refresh - always use LocalStorage
- **Missing transition:** Sudden color changes are jarring - use 200ms transitions
- **Inverting only background:** Must also invert text, borders, and shadows for proper contrast

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Theme detection | Custom system preference logic | matchMedia API | Standard browser API |
| Color transitions | JavaScript-based color interpolation | CSS transitions on root | 60fps, smoother |
| CSS variable management | Duplicate variable definitions | Single source with theme selector | Maintainability |

## Common Pitfalls

### Pitfall 1: Hardcoded Colors in Components
**What goes wrong:** Components with hardcoded #ffffff or #000000 ignore theme changes
**Why it happens:** Copy-pasting from dark theme without using CSS variables
**How to avoid:** Audit components with grep for hex colors, replace with var(--*) references
**Warning signs:** Components stay white on light theme

### Pitfall 2: Missing Border/Border-Color Adaptation
**What goes wrong:** Light theme borders are invisible or too harsh
**Why it happens:** Dark theme uses rgba(255,255,255,0.1), light needs rgba(0,0,0,0.1)
**How to avoid:** Use the UI SPEC border values exactly - they are inverted from dark theme
**Warning signs:** Cards/inputs have no visible boundaries on light theme

### Pitfall 3: Shadow Visibility on Light Background
**What goes wrong:** Dark shadows look harsh on light backgrounds
**Why it happens:** Same shadow values used for both themes
**How to avoid:** Light theme uses reduced opacity shadows (0.05-0.15 vs 0.3-0.5)
**Warning signs:** Cards cast harsh black shadows on light theme

### Pitfall 4: Theme Flash on Page Load
**What goes wrong:** User sees wrong theme for a split second before JavaScript loads
**Why it happens:** Theme applied after React/Vue hydration
**How to avoid:** Add inline script in HTML head to set data-theme before body renders

## Code Examples

### Light Theme CSS Variables
```css
/* Source: UI SPEC 02-UI-SPEC.md */
[data-theme="light"] {
  /* Background Colors */
  --bg-primary: #fafafa;
  --bg-secondary: #ffffff;
  --bg-tertiary: #f4f4f5;
  --bg-elevated: #ffffff;
  --bg-card: rgba(255, 255, 255, 0.8);
  --bg-card-hover: rgba(255, 255, 255, 0.95);

  /* Border Colors */
  --border-subtle: rgba(0, 0, 0, 0.04);
  --border-default: rgba(0, 0, 0, 0.1);
  --border-hover: rgba(0, 0, 0, 0.16);

  /* Text Colors */
  --text-primary: #18181b;
  --text-secondary: #52525b;
  --text-tertiary: #a1a1aa;
  --text-disabled: #d4d4d8;

  /* Glass Effect */
  --glass-bg: rgba(255, 255, 255, 0.8);
  --glass-border: rgba(0, 0, 0, 0.08);

  /* Shadows - reduced opacity */
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 8px 24px rgba(0, 0, 0, 0.15);
}
```

### Icon Color Variables
```css
/* Source: UI SPEC - Icon Colors section */
[data-theme="light"] {
  /* Icon colors - soft, non-dazzling */
  --icon-default: #52525b;
  --icon-hover: #18181b;
  --icon-active: #6366f1;
  --icon-disabled: #d4d4d8;
}
```

### Theme Toggle Transition
```css
/* Add to :root for smooth transitions */
*, *::before, *::after {
  transition: background-color 0.2s ease-in-out,
              border-color 0.2s ease-in-out,
              color 0.2s ease-in-out,
              box-shadow 0.2s ease-in-out;
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Single hardcoded theme | CSS custom properties with data-theme | Pre-existing | Enables instant theme switching |
| No persistence | LocalStorage + system preference | This phase | Persistent user preference |
| Component-level theming | Root-level CSS variables | Pre-existing | Consistent across all components |

**Current best practice:** CSS custom properties with JavaScript attribute toggling. This is the standard approach for Vue/React apps and matches what the project already uses for dark theme.

## Open Questions

1. **Where to place theme toggle?**
   - Current thinking: Add to Sidebar bottom area (beside collapse button)
   - Alternative: Settings page
   - Recommendation: Sidebar for immediate access

2. **Should glass effects work on light theme?**
   - The glass effect uses backdrop-filter which works on both themes
   - Light theme glass-bg is rgba(255,255,255,0.8) vs dark's rgba(15,15,20,0.7)
   - This is defined in UI SPEC and should work

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Vitest 4.0.18 |
| Config file | vitest.config.ts |
| Quick run command | `npm run test` |
| Full suite command | `npm run test` (no separate full suite) |

### Phase Requirements Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|---------------|
| LIGHT-01 | Light theme colors applied | Visual/Manual | None (requires visual inspection) | N/A |
| LIGHT-02 | Icon colors correct on light | Visual/Manual | None (requires visual inspection) | N/A |
| LIGHT-03 | Consistent styling between themes | Visual/Manual | None (requires visual inspection) | N/A |
| LIGHT-04 | Theme toggle works | Unit | `npm run test src/stores/uiStore.test.ts` | Yes - existing test |
| DOC-02 | Design tokens documented | Code review | None | N/A |

### Sampling Rate
- **Per task commit:** Not applicable for UI changes (manual verification)
- **Per wave merge:** Full test suite via `npm run test`
- **Phase gate:** Manual visual verification required

### Wave 0 Gaps
- None - existing test infrastructure adequate
- uiStore tests exist and can be extended for theme state

## Sources

### Primary (HIGH confidence)
- UI SPEC 02-UI-SPEC.md - Exact color values, implementation patterns
- style.css - Existing CSS variable implementation pattern
- uiStore.ts - Existing Pinia store pattern

### Secondary (MEDIUM confidence)
- MDN CSS Custom Properties - Standard theming approach
- Vue 3 + Pinia documentation - State management patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Uses existing project infrastructure
- Architecture: HIGH - CSS variables + Pinia is standard pattern
- Pitfalls: HIGH - UI SPEC documents exact values, reduces guesswork

**Research date:** 2026-03-17
**Valid until:** 90 days (CSS variables approach is stable)
