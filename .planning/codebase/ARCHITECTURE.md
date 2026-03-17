# Architecture

**Analysis Date:** 2026-03-17

## Pattern Overview

**Overall:** Tauri Desktop Application with Vue 3 Frontend

**Key Characteristics:**
- Tauri v2 with Rust backend for native Windows desktop application
- Vue 3 with Composition API for reactive frontend
- Pinia for state management
- SQLite local database via rusqlite
- Command-based IPC between frontend and backend

## Layers

**Frontend (Vue 3):**
- Purpose: User interface and state management
- Location: `src/`
- Contains: Vue components, Pinia stores, API client
- Depends on: @tauri-apps/api, pinia, UI libraries
- Used by: Browser/WebView rendering

**Backend (Rust/Tauri):**
- Purpose: Native application logic and data persistence
- Location: `src-tauri/src/`
- Contains: Commands (Tauri handlers), Database layer, Models
- Depends on: tauri, rusqlite, serde, chrono, uuid
- Used by: Frontend via Tauri invoke

**Database Layer:**
- Purpose: SQLite data persistence with thread-safe access
- Location: `src-tauri/src/db/mod.rs`
- Contains: Database struct, table initialization, CRUD operations
- Depends on: rusqlite, Mutex for thread safety
- Used by: Commands module

**Models:**
- Purpose: Data structures for serialization/deserialization
- Location: `src-tauri/src/models/mod.rs`
- Contains: Project, Task, Document, Milestone, User structs
- Depends on: serde, chrono
- Used by: Commands, Database layer

## Data Flow

**Application Startup:**
1. `src-tauri/src/main.rs` calls `projectflow_lib::run()`
2. `lib.rs` initializes logging and panic handler
3. Database is created in app data directory (exe-relative or AppData)
4. Tauri app starts with all command handlers registered

**Frontend-Backend Communication:**
1. Frontend calls `invoke('command_name', params)` from `src/lib/api.ts`
2. Tauri routes to corresponding command in `src-tauri/src/commands/mod.rs`
3. Command accesses Database via State<Database>
4. Database performs SQLite operations
5. Result serialized and returned to frontend

## Key Abstractions

**Tauri Commands:**
- Purpose: Expose Rust functions to frontend
- Examples: `create_project`, `get_tasks_by_project`, `update_task`
- Pattern: #[tauri::command] attribute with State injection

**Pinia Stores:**
- Purpose: Frontend state management
- Examples: `projectStore.ts`, `taskStore`, `documentStore`, `uiStore`
- Pattern: defineStore with composable reactive state

**Database Abstraction:**
- Purpose: Encapsulate all SQLite operations
- Location: `src-tauri/src/db/mod.rs`
- Pattern: Mutex-wrapped Connection with CRUD methods

## Entry Points

**Frontend Entry:**
- Location: `src/main.ts`
- Triggers: Vite dev server or production build
- Responsibilities: Create Vue app, mount to DOM, initialize Pinia

**Backend Entry:**
- Location: `src-tauri/src/main.rs`
- Triggers: Tauri application launch
- Responsibilities: Call lib::run(), set up Windows subsystem

**Tauri Setup:**
- Location: `src-tauri/src/lib.rs`
- Triggers: Called from main()
- Responsibilities: Initialize database, register commands, start Tauri builder

## Error Handling

**Frontend:**
- Errors caught in try/catch blocks in stores
- Displayed via UI notifications

**Backend:**
- Database errors converted to String and propagated
- Panic hook logs to env_logger
- All errors return Result<T, String> type

## Cross-Cutting Concerns

**Logging:** env_logger for Rust, console for Vue
**Validation:** Handled at command level with Option parameters
**Authentication:** Not implemented (local-only application)
**Data Storage:** SQLite database stored in app data directory

---

*Architecture analysis: 2026-03-17*
