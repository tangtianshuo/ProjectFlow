import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import Button from './Button.vue'

describe('Button', () => {
  it('renders properly with default props', () => {
    const wrapper = mount(Button)
    expect(wrapper.text()).toBe('')
    expect(wrapper.find('button').exists()).toBe(true)
    expect(wrapper.find('button').classes()).toContain('bg-gradient-to-r')
  })

  it('renders with custom content', () => {
    const wrapper = mount(Button, {
      slots: {
        default: 'Click me'
      }
    })
    expect(wrapper.text()).toBe('Click me')
  })

  it('applies variant classes correctly', () => {
    const wrapper = mount(Button, {
      props: { variant: 'gradient' }
    })
    expect(wrapper.find('button').classes()).toContain('bg-gradient-to-r')
  })

  it('applies secondary variant classes', () => {
    const wrapper = mount(Button, {
      props: { variant: 'secondary' }
    })
    expect(wrapper.find('button').classes()).toContain('bg-[#1a1a25]')
  })

  it('applies danger variant classes', () => {
    const wrapper = mount(Button, {
      props: { variant: 'danger' }
    })
    expect(wrapper.find('button').classes()).toContain('bg-red-500/20')
  })

  it('applies ghost variant classes', () => {
    const wrapper = mount(Button, {
      props: { variant: 'ghost' }
    })
    expect(wrapper.find('button').classes()).toContain('bg-transparent')
  })

  it('applies size sm classes', () => {
    const wrapper = mount(Button, {
      props: { size: 'sm' }
    })
    expect(wrapper.find('button').classes()).toContain('px-3')
    expect(wrapper.find('button').classes()).toContain('py-1.5')
  })

  it('applies size md classes', () => {
    const wrapper = mount(Button, {
      props: { size: 'md' }
    })
    expect(wrapper.find('button').classes()).toContain('px-4')
  })

  it('applies size lg classes', () => {
    const wrapper = mount(Button, {
      props: { size: 'lg' }
    })
    expect(wrapper.find('button').classes()).toContain('px-6')
    expect(wrapper.find('button').classes()).toContain('py-3')
  })

  it('handles disabled state', () => {
    const wrapper = mount(Button, {
      props: { disabled: true }
    })
    expect(wrapper.find('button').attributes('disabled')).toBe('')
    expect(wrapper.find('button').classes()).toContain('disabled:opacity-50')
  })

  it('does not have disabled attribute when not disabled', () => {
    const wrapper = mount(Button)
    expect(wrapper.find('button').attributes('disabled')).toBeUndefined()
  })
})
