import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import Input from './Input.vue'

describe('Input', () => {
  it('renders properly', () => {
    const wrapper = mount(Input)
    expect(wrapper.find('input').exists()).toBe(true)
  })

  it('renders with placeholder', () => {
    const wrapper = mount(Input, {
      props: { placeholder: 'Enter text' }
    })
    expect(wrapper.find('input').attributes('placeholder')).toBe('Enter text')
  })

  it('renders with default type text', () => {
    const wrapper = mount(Input)
    expect(wrapper.find('input').attributes('type')).toBe('text')
  })

  it('renders with custom type', () => {
    const wrapper = mount(Input, {
      props: { type: 'password' }
    })
    expect(wrapper.find('input').attributes('type')).toBe('password')
  })

  it('renders with modelValue', () => {
    const wrapper = mount(Input, {
      props: { modelValue: 'test value' }
    })
    expect(wrapper.find('input').attributes('value')).toBe('test value')
  })

  it('handles disabled state', () => {
    const wrapper = mount(Input, {
      props: { disabled: true }
    })
    expect(wrapper.find('input').attributes('disabled')).toBe('')
  })

  it('emits update:modelValue on input', async () => {
    const wrapper = mount(Input)
    const input = wrapper.find('input')
    await input.setValue('hello')
    expect(wrapper.emitted('update:modelValue')).toBeTruthy()
    expect(wrapper.emitted('update:modelValue')![0]).toEqual(['hello'])
  })

  it('applies default modelValue', () => {
    const wrapper = mount(Input)
    expect(wrapper.find('input').attributes('value')).toBe('')
  })
})
