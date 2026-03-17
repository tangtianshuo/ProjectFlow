# Technology Stack

**Analysis Date:** 2026-03-17

## Languages

**Primary:**
- TypeScript 5.6.2 - Frontend development
- Vue 3.5.13 - UI framework
- Rust - Desktop backend

**Secondary:**
- HTML/CSS - Presentation layer

## Runtime

**Environment:**
- Node.js 20+ (development)
- WebView2 (Windows rendering)
- Tauri 2 runtime

**Package Manager:**
- npm (v10+)
- Lockfile: `package-lock.json` present

## Frameworks

**Core:**
- Vue 3.5.13 - Progressive UI framework
- Tauri 2 - Desktop application framework (Rust-based)
- Pinia 3.0.4 - State management

**Testing:**
- Vitest 4.0.18 - Unit testing framework
- @vue/test-utils 2.4.6 - Vue component testing
- @testing-library/vue 8.1.0 - Behavior-driven testing

**Build/Dev:**
- Vite 6.0.3 - Build tool and dev server
- vue-tsc 2.1.10 - TypeScript checker for Vue
- TailwindCSS 4.2.1 - Utility-first CSS framework

## Key Dependencies

**UI Components:**
- @vue-flow/core 1.48.2 - Flow diagram component (Gantt charts)
- lucide-vue-next 0.577.0 - Icon library
- echarts 6.0.0 + vue-echarts 8.0.1 - Charting library

**Editor:**
- @guolao/vue-monaco-editor 1.6.0 - Code editor component
- marked 17.0.3 - Markdown parser

**Utilities:**
- @vueuse/core 14.2.1 - Vue composition utilities
- @vueuse/motion 3.0.3 - Animation library

**Tauri Plugins:**
- @tauri-apps/api 2 - Tauri JavaScript API
- @tauri-apps/plugin-opener 2 - File/URL opener

**Rust Dependencies (src-tauri/Cargo.toml):**
- tauri 2 - Core framework
- rusqlite 0.31 (bundled) - SQLite database
- tokio 1 - Async runtime
- chrono 0.4 - Date/time handling
- uuid 1 - UUID generation
- serde/serde_json - Serialization
- log/env_logger - Logging

## Configuration

**Environment:**
- Tauri handles env vars internally
- No .env files detected (desktop app, no external services)

**Build:**
- `vite.config.ts` - Vite configuration
- `tsconfig.json` - TypeScript configuration
- `tailwind.config.js` - TailwindCSS configuration
- `vitest.config.ts` - Test configuration
- `src-tauri/tauri.conf.json` - Tauri configuration

## Platform Requirements

**Development:**
- Node.js 20+
- Rust 1.70+
- Windows WebView2 runtime

**Production:**
- Windows desktop (primary target)
- Builds to .exe via Tauri

---

*Stack analysis: 2026-03-17*
