---
phase: 02-light-theme
plan: 03
subsystem: UI Theme
tags: [light-theme, gap-closure, ui-fix]
dependency_graph:
  requires: []
  provides: [light-theme-bg-fix]
  affects: [App.vue]
tech_stack:
  added: []
  patterns: [CSS-variables, theme-aware-styling]
key_files:
  created: []
  modified:
    - src/App.vue
decisions: []
---

# Phase 2 Plan 3: Gap Closure - Light Theme Background Fix

## Summary

Fixed App.vue hardcoded dark background that prevented light theme from working. The main app container had a hardcoded `#08080c` background color that didn't switch when the user toggled to light theme. Changed it to use the CSS variable `--bg-primary` which is already defined in style.css and switches between dark (#08080c) and light (#fafafa) based on theme.

## Task Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Fix App.vue hardcoded background color | 6613123 | src/App.vue |

## Changes Made

**Task 1: Fix App.vue hardcoded background color**

- Changed line 21 in `src/App.vue` from:
  ```vue
  <div class="flex min-h-screen bg-[#08080c]">
  ```
  To:
  ```vue
  <div class="flex min-h-screen bg-[var(--bg-primary)]">
  ```

- Uses existing CSS variable `--bg-primary` from `style.css`:
  - Dark theme (default): `--bg-primary: #08080c`
  - Light theme: `--bg-primary: #fafafa`

## Acceptance Criteria Verification

- [x] Line 21 contains "bg-[var(--bg-primary)]" instead of "bg-[#08080c]"
- [x] No other hardcoded background colors remain in the div
- [x] The class attribute still contains "flex min-h-screen"

## Verification

After this fix:
1. The main app background changes from dark (#08080c) to light (#fafafa) when users toggle to light theme
2. The theme toggle in Sidebar.vue works as expected
3. No hardcoded dark colors remain in the main container

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check

- [x] App.vue modified with theme-aware background
- [x] Commit 6613123 exists
- [x] Verification criteria met

## Self-Check: PASSED
