# Coding Conventions

**Analysis Date:** 2026-03-17

## Naming Patterns

**Files:**
- Components: PascalCase with descriptive names (e.g., `Button.vue`, `GanttChart.vue`, `TaskBoard.vue`)
- Stores: camelCase with `Store` suffix (e.g., `uiStore.ts`, `documentStore.ts`, `projectStore.ts`)
- Utilities/API: camelCase (e.g., `api.ts`)

**Functions:**
- Vue component functions: camelCase (e.g., `handleClick`, `toggleSidebar`)
- Store actions: camelCase (e.g., `setView`, `selectProject`)

**Variables:**
- Local variables: camelCase (e.g., `currentView`, `taskViewMode`)
- Props: camelCase (e.g., `variant`, `disabled`, `loading`)
- Refs: camelCase (e.g., `sidebarCollapsed`)

**Types:**
- Interfaces: PascalCase (e.g., `Project`, `Task`, `CreateProjectRequest`)
- Type aliases: PascalCase (e.g., `ViewMode`, `TaskViewMode`)

## Code Style

**Formatting:**
- Tool: Not explicitly configured (using default Vue/Vite patterns)
- Indentation: 2 spaces (based on source files)
- Quotes: Double quotes for strings in TypeScript

**Linting:**
- Not explicitly configured (no ESLint config detected)
- TypeScript strict mode enabled (`strict: true`)
- Unused variables and parameters flagged (`noUnusedLocals`, `noUnusedParameters`)

## Import Organization

**Order (observed pattern):**
1. Vue built-ins (e.g., `computed`, `ref`, `defineProps`, `defineEmits`)
2. External libraries (e.g., `@vue/test-utils`, `pinia`)
3. Internal modules (e.g., `./Button.vue`, `../stores/uiStore`)

**Path Aliases:**
- `@` maps to `src/` directory (configured in `vite.config.ts` and `vitest.config.ts`)

Example from `src/stores/uiStore.ts`:
```typescript
import { defineStore } from "pinia";
import { ref } from "vue";
```

## Error Handling

**Patterns:**
- Conditional guards in handlers (e.g., `if (!props.disabled && !props.loading)`)
- Type-safe optional parameters using TypeScript
- Null coalescing where needed

Example from `src/components/ui/Button.vue`:
```typescript
function handleClick(event: MouseEvent) {
  if (!props.disabled && !props.loading) {
    emit("click", event);
  }
}
```

## Logging

**Framework:** Console (no logging library detected)

**Patterns:**
- Minimal logging in production code
- Focus on silent operations with user feedback via UI

## Comments

**When to Comment:**
- Type definitions with JSDoc comments for API interfaces (e.g., `src/lib/api.ts`)

**JSDoc/TSDoc:**
- Used for API type definitions

Example from `src/lib/api.ts`:
```typescript
// Types
export interface Project {
  id: string;
  name: string;
  // ...
}
```

## Function Design

**Size:** Keep components focused with computed properties for complex logic

**Parameters:** Use TypeScript interfaces for request/response types

**Return Values:** Explicit return types in API functions (e.g., `Promise<Project>`)

Example from `src/lib/api.ts`:
```typescript
create: (data: CreateProjectRequest): Promise<Project> =>
  invoke("create_project", { ... }),
```

## Module Design

**Exports:**
- Named exports for stores and API modules
- Default exports for Vue components

**Barrel Files:** Not detected (direct imports used)

## Vue Component Patterns

**Script Setup:**
- Use `<script setup lang="ts">` for all Vue components
- Use `withDefaults` for prop defaults
- Use `defineEmits` for event definitions

Example from `src/components/ui/Button.vue`:
```vue
<script setup lang="ts">
interface Props {
  variant?: "primary" | "secondary" | "danger" | "ghost" | "gradient" | "outline";
  size?: "sm" | "md" | "lg";
  disabled?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  variant: "primary",
  size: "md",
  disabled: false,
});

const emit = defineEmits<{
  (e: "click", event: MouseEvent): void;
}>();
</script>
```

## State Management

**Framework:** Pinia

**Pattern:** Composition API style stores with `defineStore`

Example from `src/stores/uiStore.ts`:
```typescript
export const useUiStore = defineStore("ui", () => {
  const currentView = ref<ViewMode>("dashboard");
  // ...
  return { currentView, setView, toggleSidebar };
});
```

---

*Convention analysis: 2026-03-17*
