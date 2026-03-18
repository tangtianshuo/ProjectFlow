# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ProjectFlow is a Windows desktop project management application built with Tauri v2, Vue 3, and TypeScript. It provides project management features including projects, tasks, documents, milestones, and a recycle bin for soft-deleted items.

## Common Commands

```bash
# Development
npm run dev                 # Start Vite dev server (frontend only)
npm run tauri dev           # Start full Tauri dev mode

# Build
npm run tauri build         # Build production desktop app

# Testing
npm run test                # Run Vitest tests
npm run test:watch          # Run tests in watch mode
npm run test:coverage       # Run tests with coverage
```

## Architecture

### Frontend (Vue 3 + TypeScript)
- **State Management**: Pinia stores in `src/stores/` (projectStore, documentStore, uiStore)
- **API Layer**: `src/lib/api.ts` - Type-safe wrappers around Tauri invoke calls
- **Components**: Organized in `src/components/`:
  - `ui/` - Reusable UI components (Button, Input, Select, Modal, Icon)
  - `features/` - Feature-specific components (projects, tasks, documents, dashboard, recycleBin)
  - `layout/` - Layout components (Sidebar)
- **Dependencies**: Vue Flow (task board), ECharts (charts), Monaco Editor (documents), marked (markdown), Lucide (icons)

### Backend (Rust + Tauri v2)
- **Database**: SQLite via rusqlite (bundled), stored in app data directory
  - Tables: projects, tasks, documents, milestones, users, task_dependencies
- **Commands**: `src-tauri/src/commands/mod.rs` - Tauri command handlers
- **Models**: `src-tauri/src/models/` - Rust structs
- **Database Layer**: `src-tauri/src/db/mod.rs` - SQLite operations with soft delete

### IPC Communication
Frontend calls Rust commands via `@tauri-apps/api/core::invoke()`. All API functions are exported from `src/lib/api.ts`.

### Database Storage
- Primary: `./data/projectflow.db` (next to exe for distribution)
- Fallback: AppData directory
- Soft delete: Records marked with `deleted_at` timestamp instead of hard deletion

## CI/CD

GitHub Actions workflow in `.github/workflows/release.yml` builds and releases on version tags (v*) for Windows and macOS.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
