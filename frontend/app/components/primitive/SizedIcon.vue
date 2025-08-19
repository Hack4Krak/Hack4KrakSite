<script setup lang="ts">
type IconSize = 'small' | 'medium' | 'large'

const props = withDefaults(defineProps<{
  format?: IconSize
  name: string
}>(), {
  format: 'small',
})

// const size = computed(() => props.format === 'large' ? '7' : '4')
const size = computed(() => {
  switch (props.format) {
    case 'large':
      return '7'
    case 'medium':
      return '5'
    default:
      return '4'
  }
})
</script>

<template>
  <Icon
    :name="name" :style="{
      width: `calc(var(--spacing) * ${size})`,
      height: `calc(var(--spacing) * ${size})`,
    }"
  />
</template>
