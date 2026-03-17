# Testing Patterns

**Analysis Date:** 2026-03-17

## Test Framework

**Runner:**
- Vitest 4.0.18
- Config: `vitest.config.ts`

**Assertion Library:**
- Vitest built-in `expect`

**Run Commands:**
```bash
npm test              # Run all tests (vitest run)
npm run test:watch    # Watch mode (vitest)
npm run test:coverage # Coverage (vitest run --coverage)
```

## Test File Organization

**Location:**
- Co-located with source files
- Pattern: `{filename}.test.ts` or `{filename}.spec.ts`

**Naming:**
- Tests use `.test.ts` suffix
- Examples: `Button.test.ts`, `Input.test.ts`, `uiStore.test.ts`

**Structure:**
```
src/
├── components/
│   └── ui/
│       ├── Button.vue
│       └── Button.test.ts
└── stores/
    ├── uiStore.ts
    └── uiStore.test.ts
```

## Test Structure

**Suite Organization:**
- Uses `describe` blocks for test suites
- Uses `it` or `test` for individual test cases
- Uses `expect` for assertions

Example from `src/components/ui/Button.test.ts`:
```typescript
import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import Button from './Button.vue'

describe('Button', () => {
  it('renders properly with default props', () => {
    const wrapper = mount(Button)
    expect(wrapper.text()).toBe('')
    expect(wrapper.find('button').exists()).toBe(true)
  })
})
```

**Patterns:**
- `beforeEach` for setup (used in store tests)
- Direct mounting with `@vue/test-utils`

Example from `src/stores/uiStore.test.ts`:
```typescript
import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useUiStore } from './uiStore'

describe('useUiStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('initializes with default values', () => {
    const uiStore = useUiStore()
    expect(uiStore.currentView).toBe('dashboard')
  })
})
```

## Mocking

**Framework:** Vitest (built-in) + @vue/test-utils

**Patterns:**
- Vue Test Utils `mount` for component rendering
- Pinia `setActivePinia(createPinia())` for store testing

**What to Mock:**
- Pinia stores (use `setActivePinia`)
- External APIs (Tauri invoke calls not mocked in current tests)

**What NOT to Mock:**
- Vue components being tested
- Internal store logic (test the real store)

## Fixtures and Factories

**Test Data:**
- Inline test data in test files
- No separate fixture files detected

Example from `src/stores/uiStore.test.ts`:
```typescript
uiStore.selectProject('project-123')
expect(uiStore.selectedProjectId).toBe('project-123')
```

**Location:**
- Inline with test cases
- No dedicated fixtures directory

## Coverage

**Requirements:** None enforced (no coverage threshold)

**View Coverage:**
```bash
npm run test:coverage
```

**Coverage Configuration** (from `vitest.config.ts`):
```typescript
coverage: {
  provider: "v8",
  reporter: ["text", "json", "html"],
  reportsDirectory: "./coverage",
  include: ["src/**/*.{js,ts,vue}"],
  exclude: [
    "src/**/*.d.ts",
    "src/**/*.spec.{js,ts}",
    "src/**/*.test.{js,ts}",
    "src/main.ts",
    "src/App.vue",
    "src/assets/**",
    "src/components/**/index.ts",
  ],
},
```

## Test Types

**Unit Tests:**
- Component tests: Vue component rendering, props, variants
- Store tests: State management, actions, mutations

**Integration Tests:** Not detected

**E2E Tests:** Not used (no E2E framework detected)

## Common Patterns

**Async Testing:**
- Not explicitly used (current tests are synchronous)
- Vitest supports `async/await` for async operations

**Error Testing:**
- Not explicitly demonstrated in current tests

## Vitest Configuration

**Config File:** `vitest.config.ts`

```typescript
export default defineConfig({
  plugins: [vue()],
  test: {
    environment: "jsdom",
    globals: true,
    pool: "forks",
    poolOptions: {
      forks: {
        singleFork: true,
      },
    },
    include: ["src/**/*.{test,spec}.{js,ts}"],
  },
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
    },
  },
})
```

**Key Settings:**
- Environment: `jsdom` for DOM testing
- Globals: Enabled (no import needed for `describe`, `it`, `expect`)
- Single fork mode for consistent test isolation

---

*Testing analysis: 2026-03-17*
