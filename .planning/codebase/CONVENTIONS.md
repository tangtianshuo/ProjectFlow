# Coding Conventions

**Analysis Date:** 2026-03-17

## Naming Patterns

**Files:**
- Components: PascalCase (e.g., `Button.vue`, `ProjectList.vue`, `TaskBoard.vue`)
- Stores: camelCase with Store suffix (e.g., `projectStore.ts`, `uiStore.ts`)
- API: camelCase (e.g., `api.ts`)
- Tests: Original name + `.test.ts` suffix (e.g., `Button.test.ts`, `Input.test.ts`)

**Functions:**
- Vue component functions: camelCase (e.g., `handleClick`, `onInput`, `createProject`)
- Store actions: camelCase (e.g., `fetchProjects`, `createProject`, `deleteTask`)
- Event handlers: `on` prefix for event handlers (e.g., `onInput`, `onBackdropClick`)

**Variables:**
- camelCase (e.g., `newProject`, `showCreateModal`, `allProjectTasks`)
- Reactive refs: suffix with type or omit (e.g., `const showCreateModal = ref(false)`)
- Props interfaces: PascalCase with `Props` suffix (e.g., `interface Props`)

**Types:**
- Interfaces: PascalCase (e.g., `Project`, `Task`, `CreateProjectRequest`)
- Type aliases: PascalCase (e.g., `ViewMode`, `TaskViewMode`)
- Enums: Not used - using literal types or unions instead

## Code Style

**Formatting:**
- Tool: Built-in Vite + TypeScript (no explicit Prettier config found)
- Indentation: 2 spaces
- Quotes: Double quotes for strings
- Semicolons: Not used in Vue SFC

**Linting:**
- Tool: TypeScript compiler with strict mode
- Key rules in `tsconfig.json`:
  - `strict: true` - full type checking
  - `noUnusedLocals: true` - prevent unused variables
  - `noUnusedParameters: true` - prevent unused parameters
  - `noFallthroughCasesInSwitch: true` - enforce switch completeness
  - `isolatedModules: true` - ensure cross-file imports are valid

**TypeScript Configuration:**
- Target: ES2020
- Module: ESNext (bundler mode)
- Strict mode enabled

## Import Organization

**Order (within Vue SFC script):**
1. Vue core imports (e.g., `import { ref, computed } from "vue"`)
2. Store imports (e.g., `import { useProjectStore } from "../../../stores/projectStore"`)
3. API/lib imports (e.g., `import { taskApi, type Task } from "../../../lib/api"`)
4. Component imports (e.g., `import Button from "../../ui/Button.vue"`)

**Path Aliases:**
- `@` alias configured in `vite.config.ts` pointing to `src/`
- Example: `import { useUiStore } from "@/stores/uiStore"`

## Error Handling

**Patterns:**
- Try-catch blocks for async operations (see `src/components/features/projects/ProjectList.vue`):
  ```typescript
  try {
    const tasks = await taskApi.getByProject(project.id);
    allProjectTasks.value[project.id] = tasks;
  } catch (e) {
    console.error(`Failed to fetch tasks for project ${project.id}:`, e);
  }
  ```
- Store error state: `error.value = String(e)` pattern in stores
- User-facing errors: Display error message or fallback to console.error
- No global error boundary implemented

**Validation:**
- Form validation using HTML5 `required` attribute
- Programmatic validation before API calls (e.g., `if (!newProject.value.name.trim()) return`)

## Logging

**Framework:** console API

**Patterns:**
- Error logging: `console.error("Failed to create project:", e)`
- Error with context: `console.error(\`Failed to fetch tasks for project ${project.id}:\`, e)`
- No structured logging library configured

## Comments

**When to Comment:**
- No explicit comment guidelines found
- Code tends to be self-documenting with clear variable/function names
- Complex logic may lack inline comments

**JSDoc/TSDoc:**
- Not actively used
- Types defined via TypeScript interfaces

## Function Design

**Size:**
- Functions tend to be concise (< 30 lines)
- Complex operations broken into smaller helper functions (e.g., `getProgressColor`, `getProjectActionLabel`)

**Parameters:**
- TypeScript interfaces for complex objects (e.g., `CreateProjectRequest`)
- Optional parameters with `?` modifier
- Default values using `withDefaults` for props

**Return Values:**
- Explicit return types in API functions
- Vue components use `computed` for derived values

## Module Design

**Exports:**
- Named exports for types and API objects (e.g., `export interface Project`, `export const projectApi`)
- Vue components use default export via `<script setup>`

**Barrel Files:**
- Not used - imports use relative paths
- Example: `import { useProjectStore } from "../../../stores/projectStore"`

## Vue 3 Specific Conventions

**Script Setup:**
- Use `<script setup lang="ts">` for all Vue components
- Props defined with `interface Props` + `withDefaults(defineProps<Props>(), {...})`
- Emits defined with `defineEmits<{...}>()`

**Component Structure:**
- Props interfaces defined at top of script
- Reactive state with `ref()` and `computed()`
- Lifecycle hooks: `onMounted`, `onBeforeUnmount`
- Event handlers as regular functions

---

*Convention analysis: 2026-03-17*
