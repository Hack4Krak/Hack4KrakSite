import { mount } from '@vue/test-utils'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import AnimatedText from '~/components/AnimatedText.vue'

describe('animatedText', () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.clearAllTimers()
    vi.useRealTimers()
  })

  it('renders and types text', async () => {
    const wrapper = mount(AnimatedText, {
      props: { text: 'Hello', speed: 10 },
    })

    await nextTick()
    expect(wrapper.text()).toBe('H') // first character typed immediately
    await nextTick()
    vi.advanceTimersByTime(10)
    await nextTick()
    expect(wrapper.text()).toBe('He')
    vi.advanceTimersByTime(10)
    await nextTick()
    expect(wrapper.text()).toBe('Hel')
    vi.advanceTimersByTime(10)
    await nextTick()
    expect(wrapper.text()).toBe('Hell')
    vi.advanceTimersByTime(20)
    await nextTick()
    expect(wrapper.text()).toBe('Hello')
  })

  it('restarts animation when text changes', async () => {
    const wrapper = mount(AnimatedText, {
      props: { text: 'Hi', speed: 10 },
    })

    vi.advanceTimersByTime(30)
    await nextTick()
    expect(wrapper.text()).toBe('Hi')

    await wrapper.setProps({ text: 'Bye' })
    await nextTick()
    expect(wrapper.text()).toBe('B')
    vi.advanceTimersByTime(50)
    await nextTick()
    expect(wrapper.text()).toBe('Bye')
  })
})
