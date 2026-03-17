# Testing Patterns

**Analysis Date:** 2026-03-17

## Test Framework

**Runner:**
- Vitest v4.0.18
- Config: `vitest.config.ts`

**Assertion Library:**
- Vitest built-in `expect`
- Vue Test Utils for component testing

**Test Dependencies:**
- `@vue/test-utils` v2.4.6 - Vue component testing
- `@testing-library/vue` v8.1.0 - Vue testing utilities
- `@vitest/coverage-v8` v4.0.18 - V8 coverage provider
- `jsdom` v24.1.3 - DOM environment for tests

**Run Commands:**
```bash
npm run test              # Run all tests once
npm run test:watch       # Run tests in watch mode
npm run test:coverage    # Run tests with coverage
```

## Test File Organization

**Location:**
- Co-located with source files (same directory)
- Tests placed alongside components and stores

**Naming:**
- `{ComponentName}.test.ts` pattern
- Examples: `Button.test.ts`, `Input.test.ts`, `uiStore.test.ts`

**Structure:**
```
src/
├── components/
│   └── ui/
│       ├── Button.vue
│       ├── Button.test.ts    # Component tests
│       ├── Input.vue
│       └── Input.test.ts
├── stores/
│   ├── uiStore.ts
│   └── uiStore.test.ts       # Store tests
```

## Test Structure

**Component Tests (Vue Test Utils):**
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

**Store Tests (Pinia):**
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

**Patterns:**
- `describe()` blocks group related tests
- `it()` or `test()` for individual test cases
- `beforeEach()` for setup/reset
- Assertions use `expect().toBe()`, `.toBeTruthy()`, `.toEqual()`, etc.

## Mocking

**Framework:** Built-in Vitest + Vue Test Utils

**Patterns:**
- No explicit mocking framework (e.g., no vi.fn() or jest.mock)
- Components mounted with `mount()` from Vue Test Utils
- Props passed via `mount(Component, { props: {...} })`
- Slots tested via `mount(Component, { slots: {...} })`

**Mocking Example from codebase:**
```typescript
// Passing props to component
const wrapper = mount(Input, {
  props: { placeholder: 'Enter text' }
})

// Testing emitted events
const wrapper = mount(Input)
const input = wrapper.find('input')
await input.setValue('hello')
expect(wrapper.emitted('update:modelValue')).toBeTruthy()
expect(wrapper.emitted('update:modelValue')![0]).toEqual(['hello'])
```

**Pinia Store Mocking:**
```typescript
import { setActivePinia, createPinia } from 'pinia'

beforeEach(() => {
  setActivePinia(createPinia())
})
```

**What to Mock:**
- External API calls are NOT mocked in current tests
- Tests are unit tests focusing on component behavior
- Store state is reset via `beforeEach` with fresh Pinia instance

**What NOT to Mock:**
- No HTTP mocking (tests don't cover backend integration)
- No Tauri invoke mocking

## Fixtures and Factories

**Test Data:**
- Inline data in test files
- No dedicated fixture files

**Example Pattern:**
```typescript
// Props as inline data
const wrapper = mount(Button, {
  props: { variant: 'gradient', size: 'md' }
})
```

**Location:**
- No separate fixtures directory
- Tests are self-contained

## Coverage

**Requirements:** None enforced

**View Coverage:**
```bash
npm run test:coverage
```

**Configuration (vitest.config.ts):**
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
}
```

**Excluded from Coverage:**
- Type definition files (`.d.ts`)
- Test files
- Entry points (main.ts, App.vue)
- Asset files

## Test Types

**Unit Tests:**
- Component rendering tests
- Props validation tests
- Event emission tests
- Store state/behavior tests

**Integration Tests:**
- Not detected in current codebase

**E2E Tests:**
- Not used - this is a Tauri desktop app

## Common Patterns

**Async Testing:**
- Use `async/await` in test functions
- Use `await wrapper.vm.$nextTick()` for DOM updates (implicit in Vue Test Utils v2+)
- Example: `await input.setValue('hello')` handles async input

**Error Testing:**
- Not extensively tested in current test suite
- Components don't have dedicated error state tests

**Class/Attribute Testing:**
```typescript
// Check CSS classes
expect(wrapper.find('button').classes()).toContain('bg-gradient-to-r')

// Check attributes
expect(wrapper.find('input').attributes('placeholder')).toBe('Enter text')
expect(wrapper.find('input').attributes('disabled')).toBe('')
```

**State Testing:**
```typescript
// Reactive state
const uiStore = useUiStore()
uiStore.setView('projects')
expect(uiStore.currentView).toBe('projects')

// Toggle state
uiStore.toggleSidebar()
expect(uiStore.sidebarCollapsed).toBe(true)
```

---

*Testing analysis: 2026-03-17*
