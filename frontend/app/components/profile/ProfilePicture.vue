<script setup lang="ts">
const props = withDefaults(defineProps<{
  username?: string | null
  size?: 'sm' | 'lg'
  tone?: 'primary' | 'muted'
}>(), {
  size: 'lg',
  tone: 'primary',
})

const NAME_SPLIT = /[\s_-]+/

const initials = computed(() => {
  const value = props.username?.trim()

  if (!value)
    return '?'

  const parts = value.split(NAME_SPLIT).filter(Boolean)

  const chars = parts.length === 1
    ? Array.from(parts[0]!).slice(0, 2)
    : parts.slice(0, 2).map(part => Array.from(part)[0]!)

  return chars.join('').toUpperCase() || '?'
})

const pictureClass = computed(() => [
  'border-2 font-sans font-semibold uppercase',
  props.size === 'sm' ? 'size-9 text-sm' : 'mb-4 size-20 text-xl',
  props.tone === 'primary'
    ? 'border-primary text-primary'
    : 'border-surface-muted text-muted',
])
</script>

<template>
  <UAvatar
    :text="initials"
    color="neutral"
    :class="pictureClass"
    :ui="{
      root: 'rounded-none bg-surface-muted/20',
      fallback: 'font-sans font-semibold',
    }"
  />
</template>
