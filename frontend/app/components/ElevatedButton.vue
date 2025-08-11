<script setup lang="ts">
import { tv } from 'tailwind-variants'

const { variant = 'dark' } = defineProps<{
  background?: string
  variant?: 'dark' | 'light'
}>()

const innerClass = tv({
  base: '',
  variants: {
    button: {
      light: 'border-dark bg-inverted text-inverted border-dark',
      dark: 'font-pixelify bg-default border-white',
    },
  },
})
</script>

<template>
  <button
    class="bg-primary-500 text-white text-md font-bold"
    :style="{ backgroundColor: background }"
  >
    <span
      class="block px-4 py-2 shadow-cream -translate-x-1 -translate-y-1
      bg-default border-2 duration-300 hover:-translate-y-1.5
      hover:-translate-x-1.5 hover:cursor-pointer active:translate-x-0 active:translate-y-0"
      :class="innerClass({ button: variant })"
    >
      <slot />
    </span>
  </button>
</template>
