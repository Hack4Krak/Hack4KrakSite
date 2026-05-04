import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import ProfilePicture from '@/components/profile/ProfilePicture.vue'

describe('profilePicture', () => {
  it('uses the first two letters for a single-word username', () => {
    const wrapper = mount(ProfilePicture, {
      props: { username: 'Norbiros' },
    })

    expect(wrapper.text()).toBe('NO')
  })

  it('uses initials from the first two words for a multi-word username', () => {
    const wrapper = mount(ProfilePicture, {
      props: { username: 'Norbi Ros' },
    })

    expect(wrapper.text()).toBe('NR')
  })

  it('falls back to a question mark when the username is empty', () => {
    const wrapper = mount(ProfilePicture, {
      props: { username: '   ' },
    })

    expect(wrapper.text()).toBe('?')
  })
})
