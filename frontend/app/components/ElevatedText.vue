<script setup lang="ts">
import { tv } from 'tailwind-variants'

const props = defineProps<{
  colorHex: string
  height: 'popup' | 'default'
}>()

const innerClasses = tv({
  base: 'block text-md font-bold px-4 py-2 shadow-cream -translate-x-1 -translate-y-1\n'
    + 'bg-default text-white border-1\n'
    + 'duration-300',
  variants: {
    height: {
      default: '',
      popup: 'h-[20vh]',
    },
  },
  defaultVariants: {
    height: 'default',
  },
})
</script>

<template>
  <div class="elevated-button bg-white">
    <span
      :style="{ backgroundColor: props.colorHex }"
      class="block -translate-x-[0.1px] -translate-y-[1px]"
    >
      <span
        :class="innerClasses({ height: props.height })"
      >
        <slot />
      </span>
    </span>
  </div>
</template>
