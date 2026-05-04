<script setup lang="ts">
const props = defineProps<{
  username?: string | null
}>()

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
</script>

<template>
  <div
    class="size-20 bg-primary text-default font-pixelify text-2xl flex items-center justify-center mb-4"
  >
    {{ initials }}
  </div>
</template>
