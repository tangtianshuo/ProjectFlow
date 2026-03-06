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
    expect(uiStore.taskViewMode).toBe('kanban')
    expect(uiStore.sidebarCollapsed).toBe(false)
    expect(uiStore.selectedProjectId).toBe(null)
  })

  it('setView changes currentView', () => {
    const uiStore = useUiStore()
    uiStore.setView('projects')
    expect(uiStore.currentView).toBe('projects')
  })

  it('setTaskViewMode changes taskViewMode', () => {
    const uiStore = useUiStore()
    uiStore.setTaskViewMode('gantt')
    expect(uiStore.taskViewMode).toBe('gantt')
  })

  it('toggleSidebar toggles sidebarCollapsed', () => {
    const uiStore = useUiStore()
    expect(uiStore.sidebarCollapsed).toBe(false)
    uiStore.toggleSidebar()
    expect(uiStore.sidebarCollapsed).toBe(true)
    uiStore.toggleSidebar()
    expect(uiStore.sidebarCollapsed).toBe(false)
  })

  it('selectProject sets selectedProjectId', () => {
    const uiStore = useUiStore()
    uiStore.selectProject('project-123')
    expect(uiStore.selectedProjectId).toBe('project-123')
  })

  it('selectProject can set to null', () => {
    const uiStore = useUiStore()
    uiStore.selectProject('project-123')
    uiStore.selectProject(null)
    expect(uiStore.selectedProjectId).toBe(null)
  })

  it('returns all view modes', () => {
    const uiStore = useUiStore()
    uiStore.setView('documents')
    expect(uiStore.currentView).toBe('documents')
    uiStore.setView('recycleBin')
    expect(uiStore.currentView).toBe('recycleBin')
    uiStore.setView('settings')
    expect(uiStore.currentView).toBe('settings')
  })
})
