<script setup lang="ts">
import { tv } from 'tailwind-variants'

const props = withDefaults(defineProps<{
  size?: 'sm' | 'md' | 'lg'
}>(), {
  size: 'md',
})

const logoVariants = tv({
  base: 'mr-4',
  variants: {
    size: {
      sm: 'size-6',
      md: 'size-9',
      lg: 'size-12',
    },
  },
})

const pVariants = tv({
  base: 'font-semibold font-pixelify',
  variants: {
    size: {
      sm: 'text-lg',
      md: 'text-xl',
      lg: 'text-2xl',
    },
  },
})
</script>

<template>
  <div class="flex items-center text-default">
    <Logo :class="logoVariants({ size: props.size })" />
    <p :class="pVariants({ size: props.size })">
      Hack4Krak
    </p>
  </div>
</template>
