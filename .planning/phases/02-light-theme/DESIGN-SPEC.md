# Light Theme Design Specification

## Overview
This document describes the light theme design tokens for ProjectFlow, maintaining visual consistency with the dark theme.

## Color Palette

### Background Colors
| Token | Hex | Usage |
|-------|-----|-------|
| --bg-primary | #fafafa | Main page background |
| --bg-secondary | #ffffff | Cards, sidebar, modal surfaces |
| --bg-tertiary | #f4f4f5 | Input backgrounds, hover states |
| --bg-elevated | #ffffff | Dropdown menus, popovers |

### Border Colors
| Token | Hex | Usage |
|-------|-----|-------|
| --border-subtle | rgba(0, 0, 0, 0.04) | Very subtle borders |
| --border-default | rgba(0, 0, 0, 0.1) | Default borders |
| --border-hover | rgba(0, 0, 0, 0.16) | Hover state borders |

### Text Colors
| Token | Hex | Usage |
|-------|-----|-------|
| --text-primary | #18181b | Main content, headings |
| --text-secondary | #52525b | Descriptions, labels |
| --text-tertiary | #a1a1aa | Placeholders, hints |
| --text-disabled | #d4d4d8 | Disabled states |

### Accent Colors
| Token | Hex | Usage |
|-------|-----|-------|
| --accent-primary | #6366f1 | Primary buttons, links |
| --accent-primary-hover | #4f46e5 | Hover state |
| --accent-secondary | #8b5cf6 | Secondary accent |
| --accent-cyan | #06b6d4 | Info, progress |
| --accent-green | #10b981 | Success states |
| --accent-amber | #f59e0b | Warning states |
| --accent-red | #ef4444 | Error, destructive |

## Icon Colors
| Context | Hex |
|---------|-----|
| Default | #52525b |
| Hover | #18181b |
| Active | #6366f1 |
| Disabled | #d4d4d8 |

## Spacing Scale
Same as dark theme (4px multiples): 4, 8, 16, 24, 32, 48, 64

## Typography
Same as dark theme: Inter font, 14px body, 12px labels

## Shadow Values
| Token | Value |
|-------|-------|
| --shadow-sm | 0 1px 2px rgba(0, 0, 0, 0.05) |
| --shadow-md | 0 4px 12px rgba(0, 0, 0, 0.1) |
| --shadow-lg | 0 8px 24px rgba(0, 0, 0, 0.15) |

## Theme Switching
- Toggle location: Sidebar
- Persistence: LocalStorage key 'projectflow-theme'
- Default: System preference (dark fallback)
- Transition: 200ms ease-in-out
