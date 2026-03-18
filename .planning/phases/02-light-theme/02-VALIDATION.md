---
phase: 2
slug: light-theme
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-17
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Vitest (existing from Phase 1) |
| **Config file** | vitest.config.ts |
| **Quick run command** | `npm run test` |
| **Full suite command** | `npm run test:coverage` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `npm run test`
- **After every plan wave:** Run `npm run test:coverage`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

This is a UI theme phase with no backend logic changes. All verification is visual/CSS-based and manual.

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 01 | 1 | LIGHT-01 | manual | N/A | N/A | ⬜ pending |
| 02-01-02 | 01 | 1 | LIGHT-02 | manual | N/A | N/A | ⬜ pending |
| 02-01-03 | 01 | 1 | LIGHT-03 | manual | N/A | N/A | ⬜ pending |
| 02-02-01 | 02 | 2 | LIGHT-04 | manual | N/A | N/A | ⬜ pending |
| 02-02-02 | 02 | 2 | DOC-02 | manual | N/A | N/A | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

This is a visual/UI phase. No automated tests needed:
- No backend logic changes
- No API changes
- All verification is CSS-based (colors, spacing, typography)

*Existing infrastructure covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Light theme background colors applied | LIGHT-01 | CSS-based visual verification | Open app, verify background is #fafafa/#ffffff |
| Icon colors soft/not dazzling | LIGHT-02 | Visual verification | Check icons are #52525b default, #18181b hover |
| Theme consistency | LIGHT-03 | Visual comparison | Compare light vs dark theme side by side |
| Theme toggle functional | LIGHT-04 | User interaction | Click toggle, verify theme switches |
| Design doc created | DOC-02 | File existence | Verify DESIGN-SPEC.md exists |

*Note: These manual verifications align with the UI SPEC requirements.*

---

## Validation Sign-Off

- [ ] All tasks have manual verification criteria documented
- [ ] Wave 0 not required for UI theme phases
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending 2026-03-17
