---
phase: 1
slug: dark-ui-optimization
type: design-specification
created: 2026-03-17
---

# Dark Theme Design Specification

> Single source of truth for all dark theme styling decisions in ProjectFlow.

---

## Color Tokens

### Text Colors

| Token | Hex | Usage |
|-------|-----|-------|
| `--text-primary` | #f4f4f5 | Main content, headings |
| `--text-secondary` | #a1a1aa | Descriptions, labels |
| `--text-tertiary` | #71717a | Placeholders, hints |
| `--text-disabled` | #52525b | Disabled states |

### Background Colors

| Token | Hex | Usage |
|-------|-----|-------|
| `--bg-primary` | #08080c | Main background |
| `--bg-secondary` | #0f0f14 | Cards, sidebar |
| `--bg-tertiary` | #16161d | Elevated surfaces, inputs |
| `--bg-elevated` | #1c1c26 | Hover states, modals |

### Accent Colors

| Token | Hex | Usage |
|-------|-----|-------|
| `--accent-primary` | #6366f1 | Primary actions, links, focus |
| `--accent-secondary` | #8b5cf6 | Secondary accent |
| `--accent-cyan` | #22d3ee | Progress bars, highlights |
| `--accent-green` | #22c55e | Success states |
| `--accent-red` | #ef4444 | Destructive actions, errors |

### Border Colors

| Token | Value | Usage |
|-------|-------|-------|
| `--border-default` | rgba(255, 255, 255, 0.1) | Default borders |
| `--border-hover` | rgba(255, 255, 255, 0.15) | Hover borders |

---

## Typography

| Role | Size | Weight | Line Height |
|------|------|--------|-------------|
| Body | 14px | 400 | 1.6 |
| Label | 12px | 400 | 1.4 |
| Heading | 18px | 600 | 1.3 |
| Display | 24px | 600 | 1.2 |

**Font Family:** Inter, system-ui, -apple-system, sans-serif

---

## Spacing Scale

| Token | Value | Usage |
|-------|-------|-------|
| xs | 4px | Icon gaps, inline padding |
| sm | 8px | Compact element spacing |
| md | 16px | Default element spacing |
| lg | 24px | Section padding |
| xl | 32px | Layout gaps |
| 2xl | 48px | Major section breaks |

---

## Component Standards

### Icon Component

- **Default color:** `text-[var(--text-secondary)]` (#a1a1aa)
- **Hover color:** `text-[var(--text-primary)]` (#f4f4f5)
- **Active/Selected:** `text-[var(--accent-primary)]` (#6366f1)
- **Disabled:** `text-[var(--text-disabled)]` (#52525b)

### Button Variants

| Variant | Background | Text | Border |
|---------|------------|------|--------|
| primary | var(--bg-elevated) | var(--text-primary) | var(--border-default) |
| secondary | var(--bg-tertiary) | var(--text-secondary) | var(--border-default) |
| danger | red-500/15 | var(--accent-red) | red-500/25 |
| ghost | transparent | var(--text-secondary) | none |
| gradient | gradient-primary | white | none |
| outline | transparent | var(--text-primary) | var(--border-default) |

### Input/Select Components

- **Background:** `var(--bg-tertiary)`
- **Text:** `var(--text-primary)`
- **Placeholder:** `var(--text-tertiary)`
- **Border:** `var(--border-default)`
- **Focus border:** `var(--accent-primary)`
- **Focus ring:** `var(--accent-primary)/20`

---

## Priority Tag Colors

| Priority | Background | Text |
|----------|------------|------|
| Highest / High (3,4) | rgba(248, 113, 113, 0.15) | #f87171 |
| Medium (2) | rgba(251, 191, 36, 0.15) | #fbbf24 |
| Low / Lowest (0,1) | rgba(34, 211, 238, 0.15) | #22d3ee |

**Classes:**
- Highest/High: `bg-red-500/15 text-red-400`
- Medium: `bg-amber-500/15 text-amber-400`
- Low/Lowest: `bg-cyan-500/15 text-cyan-400`

---

## Progress Bar

- **Start color:** `#6366f1` (var(--accent-primary))
- **End color:** `#22d3ee` (var(--accent-cyan))
- **Direction:** left-to-right
- **CSS:** `linear-gradient(90deg, var(--accent-primary) 0%, var(--accent-cyan) 100%)`
- **Animation:** `transition-all duration-500 ease-out`
- **Complete state:** `var(--accent-green)` (solid green)

---

## Implementation Notes

1. All components should use CSS variables instead of hardcoded colors
2. Use Tailwind arbitrary values syntax: `bg-[var(--variable)]`
3. Maintain consistency across all UI components
4. Test dark theme appearance in production build

---

**Last Updated:** 2026-03-17
