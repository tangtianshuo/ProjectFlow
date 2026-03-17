# External Integrations

**Analysis Date:** 2026-03-17

## APIs & External Services

**External APIs:** None detected
- No HTTP client libraries (axios, fetch wrappers)
- No third-party REST APIs

**Desktop Capabilities (Tauri):**
- @tauri-apps/plugin-opener - Open external URLs and files in system default applications

## Data Storage

**Databases:**
- SQLite (rusqlite 0.31 with bundled feature)
  - Connection: Local file-based database
  - Location: Application data directory (managed by Tauri)
  - No remote database

**File Storage:**
- Local filesystem only
- Uses Tauri APIs for file dialogs and system integration

**Caching:** None
- No Redis, Memcached, or in-memory caching layer

## Authentication & Identity

**Auth Provider:** None
- No authentication system implemented
- Single-user desktop application
- No user accounts or login flows

## Monitoring & Observability

**Error Tracking:** None
- No Sentry, Bugsnag, or similar services

**Logs:**
- Rust: `log` + `env_logger` crates (console logging)
- Frontend: Browser console only

## CI/CD & Deployment

**Hosting:** Not applicable
- Desktop application (not web-hosted)

**CI Pipeline:**
- GitHub Actions present (`.github/` directory)
- Configuration not analyzed (outside scope)

## Environment Configuration

**Required env vars:** None
- Desktop app with local-only functionality

**Secrets location:** Not applicable
- No external secrets required
- No cloud services connected

## Webhooks & Callbacks

**Incoming:** None
- No webhook endpoints

**Outgoing:** None
- No external API calls
- No webhook integrations

---

## Summary

This is a **standalone desktop application** with no external integrations. All data is stored locally in SQLite. The application uses:

- Tauri 2 for native desktop capabilities
- Local SQLite database for persistence
- No cloud services, APIs, or authentication
- No webhooks or external callbacks

This architecture prioritizes offline-first operation and local data ownership.

---

*Integration audit: 2026-03-17*
