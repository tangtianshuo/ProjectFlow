# Codebase Concerns

**Analysis Date:** 2026-03-17

## Tech Debt

**Database Schema Migration:**
- Issue: Migration uses `ALTER TABLE ... .ok()` which silently ignores errors
- Files: `src-tauri/src/db/mod.rs` (lines 98-100, 143)
- Impact: Schema changes may fail silently, leading to inconsistent database state
- Fix approach: Implement proper versioned migrations with explicit migration scripts

**No Database Migrations System:**
- Issue: Schema created on every startup with `CREATE TABLE IF NOT EXISTS`
- Files: `src-tauri/src/db/mod.rs` (lines 37-151)
- Impact: Cannot safely evolve schema, no rollback capability
- Fix approach: Implement a migration runner tracking applied migrations in a _migrations table

**SQLite Concurrency Limitation:**
- Issue: Uses single `Mutex<Connection>` for all database operations
- Files: `src-tauri/src/db/mod.rs` (lines 19-21)
- Impact: Blocks concurrent requests, poor scalability for multi-user scenarios
- Fix approach: Consider connection pooling or switch to PostgreSQL for multi-instance deployments

**Missing Database Indexes:**
- Issue: Indexes only on `project_id`, `parent_id` - no index on `deleted_at` or `status`
- Files: `src-tauri/src/db/mod.rs` (lines 146-148)
- Impact: Queries with `WHERE deleted_at IS NULL` perform full table scans
- Fix approach: Add indexes: `idx_projects_deleted_at`, `idx_tasks_deleted_at`, `idx_tasks_status`

## Known Bugs

**Date Parsing Silently Fails:**
- Symptoms: Dates from database may be silently converted to current time if parsing fails
- Files: `src-tauri/src/db/mod.rs` (lines 194-196, 218-220, 359-361)
- Trigger: Invalid date format stored in database
- Workaround: None - data silently corrupted

**Dynamic SQL Parameter Binding:**
- Issue: Dynamic query building with positional parameters is error-prone
- Files: `src-tauri/src/db/mod.rs` (lines 293-298, 420-424, 664-668)
- Trigger: Complex update operations with multiple optional fields
- Fix approach: Use a query builder library or simplify API to require all fields

**Frontend Error Swallowing:**
- Issue: Errors caught and converted to strings but rarely displayed to users
- Files: `src/stores/projectStore.ts` (lines 41, 53, 67, etc.)
- Trigger: Any API call failure results in silent failure with error in store
- Workaround: Users have no feedback on operation failures

## Security Considerations

**No Input Validation:**
- Risk: Backend commands accept any input without validation
- Files: `src-tauri/src/commands/mod.rs`
- Current mitigation: None
- Recommendations: Add input validation (non-empty names, valid dates, length limits)

**No Authentication/Authorization:**
- Risk: Any user with access to the app can access all data
- Files: Entire application
- Current mitigation: None (single-user local app)
- Recommendations: Not critical for local desktop app, but document this limitation

**SQL Injection Risk via Dynamic Queries:**
- Risk: String concatenation in SQL building could be exploited
- Files: `src-tauri/src/db/mod.rs` (lines 293, 420, 664)
- Current mitigation: Parameterized queries used, but dynamic SQL construction is fragile
- Recommendations: Use query builder or typed SQL

**Foreign Key Cascade Issues:**
- Risk: `ON DELETE SET NULL` leaves orphaned records when parent is deleted
- Files: `src-tauri/src/db/mod.rs` (lines 76-77, 92)
- Current mitigation: None
- Recommendations: Review deletion strategy - may want cascade delete or hard delete

## Performance Bottlenecks

**No Query Optimization:**
- Problem: No pagination on list queries, loads all records into memory
- Files: `src-tauri/src/db/mod.rs` (get_all_projects, get_all_documents)
- Cause: Simple implementation without limits
- Improvement path: Add LIMIT/OFFSET pagination and cursor-based pagination for large datasets

**Computed Properties in Frontend:**
- Problem: `projectProgress` computed recalculates on every access
- Files: `src/stores/projectStore.ts` (lines 21-33)
- Cause: No caching, iterates all tasks for each project
- Improvement path: Memoize progress calculation, update only when tasks change

**Eager Loading Missing:**
- Problem: Projects and tasks fetched separately
- Files: `src/stores/projectStore.ts` (fetchProjects does not fetch tasks)
- Cause: No relationship loading strategy
- Improvement path: Implement proper data loading with relationships

## Fragile Areas

**Hardcoded Status Values:**
- Files: `src/stores/projectStore.ts` (lines 13, 17, 28), `src/lib/api.ts` (status: 0, 1, 2, 3)
- Why fragile: No enum or constants - easy to use wrong values
- Safe modification: Create TypeScript enums and Rust constants for status values
- Test coverage: None

**Error Handling with unwrap():**
- Files: `src-tauri/src/db/mod.rs` (lines 38, 155, 181, etc.)
- Why fragile: `unwrap()` on Mutex lock and database results can panic
- Safe modification: Proper error propagation with `?` operator and Result types

**Frontend State Mutations:**
- Files: `src/stores/projectStore.ts` (lines 64, 98, 128, 145)
- Why fragile: Direct array mutations without validation
- Safe modification: Add validation before state updates

## Scaling Limits

**Single User Local Storage:**
- Current capacity: Designed for single user, local SQLite
- Limit: No multi-user support, data lives on one machine
- Scaling path: Not applicable for current design (local desktop app)

**Database Size:**
- Current capacity: SQLite performs well up to ~10GB
- Limit: Large document content could bloat database
- Scaling path: Consider external file storage for documents, keep only metadata in SQLite

## Dependencies at Risk

**Tauri 2.x:**
- Risk: Relatively new major version, ecosystem still maturing
- Impact: Plugin compatibility issues, breaking changes
- Migration plan: Monitor Tauri 2.x releases, test thoroughly before upgrading

**Vue 3 + Pinia 3:**
- Risk: Rapid evolution, some API changes between minor versions
- Impact: Dependency updates may require code changes
- Migration plan: Pin exact versions in package.json, test upgrades thoroughly

## Missing Critical Features

**No Data Backup/Export:**
- Problem: No way to export data or create backups
- Blocks: Disaster recovery, data portability

**No Search Functionality:**
- Problem: No global search across projects, tasks, documents
- Blocks: Finding content in large datasets

**No Offline Indicator:**
- Problem: App appears to work but API calls may silently fail
- Blocks: User awareness of connection issues

**Settings Page Incomplete:**
- Problem: Settings view shows placeholder text
- Files: `src/App.vue` (lines 42-45)
- Blocks: User preferences, theme customization

## Test Coverage Gaps

**Frontend Tests:**
- What's not tested: Most components, stores, API integration
- Files: Only `src/components/ui/Button.test.ts`, `src/components/ui/Input.test.ts`, `src/stores/uiStore.test.ts`
- Risk: UI regressions go undetected
- Priority: High

**Backend Tests:**
- What's not tested: No Rust unit or integration tests
- Files: None in `src-tauri/src/`
- Risk: Database logic bugs, edge cases in data handling
- Priority: High

**E2E Tests:**
- What's not tested: No end-to-end tests
- Risk: Full user workflows may break
- Priority: Medium

**Store Tests:**
- What's not tested: projectStore, documentStore have no tests
- Files: `src/stores/projectStore.ts`, `src/stores/documentStore.ts`
- Risk: State management bugs
- Priority: High

---

*Concerns audit: 2026-03-17*
