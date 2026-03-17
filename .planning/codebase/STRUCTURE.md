# Codebase Structure

**Analysis Date:** 2026-03-17

## Directory Layout

```
project-root/
├── src/                    # Vue 3 frontend source
├── src-tauri/              # Rust backend source
├── public/                 # Static assets
├── docs/                   # Documentation
├── coverage/               # Test coverage reports
├── dist/                   # Build output
├── package.json            # Frontend dependencies
├── vite.config.ts          # Vite configuration
├── tsconfig.json           # TypeScript configuration
├── tailwind.config.js      # Tailwind CSS config
├── vitest.config.ts        # Test configuration
└── .github/                # GitHub workflows
```

## Directory Purposes

**src/ (Frontend):**
- Purpose: Vue 3 application source code
- Contains: Components, stores, API client, styles
- Key files: `main.ts`, `App.vue`, `style.css`

**src-tauri/src/ (Backend):**
- Purpose: Rust Tauri application
- Contains: Commands, database, models
- Key files: `main.rs`, `lib.rs`, `commands/mod.rs`, `db/mod.rs`, `models/mod.rs`

**src/components/:**
- Purpose: Vue components organized by type
- Structure:
  - `ui/` - Reusable UI components (Button, Input, Modal, Icon, Select)
  - `layout/` - Layout components (Sidebar)
  - `features/` - Feature-specific components

**src/stores/:**
- Purpose: Pinia state management
- Files: `projectStore.ts`, `documentStore.ts`, `uiStore.ts`

**src/lib/:**
- Purpose: Utility libraries
- Files: `api.ts` - Tauri API wrapper

## Key File Locations

**Entry Points:**
- `src/main.ts` - Vue app initialization
- `src-tauri/src/main.rs` - Rust application entry
- `src-tauri/src/lib.rs` - Tauri setup and command registration

**Configuration:**
- `package.json` - Frontend dependencies and scripts
- `vite.config.ts` - Vite build config
- `tsconfig.json` - TypeScript config
- `tailwind.config.js` - Tailwind CSS
- `vitest.config.ts` - Test runner config
- `src-tauri/Cargo.toml` - Rust dependencies

**Core Logic:**
- `src-tauri/src/commands/mod.rs` - Tauri command handlers
- `src-tauri/src/db/mod.rs` - Database operations
- `src-tauri/src/models/mod.rs` - Data models
- `src/lib/api.ts` - Frontend API calls

## Naming Conventions

**Files:**
- Vue components: PascalCase (e.g., `GanttChart.vue`, `TaskBoard.vue`)
- TypeScript modules: camelCase (e.g., `api.ts`, `documentStore.ts`)
- Rust modules: snake_case (e.g., `mod.rs`, `commands/mod.rs`)

**Directories:**
- Components: lowercase with subdirectories for organization
- Stores: camelCase (e.g., `projectStore.ts`)
- Rust modules: snake_case

## Where to Add New Code

**New Feature:**
- Frontend: `src/components/features/[feature-name]/`
- Backend: Add commands to `src-tauri/src/commands/mod.rs`

**New Component/Module:**
- Implementation: `src/components/` subdirectory
- Tests: Co-located `.test.ts` files

**New Store:**
- Location: `src/stores/[name]Store.ts`

**New Tauri Command:**
- Handler: `src-tauri/src/commands/mod.rs`
- Database method: `src-tauri/src/db/mod.rs`
- Model (if needed): `src-tauri/src/models/mod.rs`

## Special Directories

**src-tauri/target/:**
- Purpose: Rust build output (generated)
- Committed: No (in .gitignore)

**coverage/:**
- Purpose: Test coverage reports
- Generated: Yes (by vitest)
- Committed: No

**dist/:**
- Purpose: Production build output
- Generated: Yes (by vite)
- Committed: No

---

*Structure analysis: 2026-03-17*
