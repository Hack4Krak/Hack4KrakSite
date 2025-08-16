<script setup lang="ts">
type IconSize = 'small' | 'large'

const props = withDefaults(defineProps<{
  format?: IconSize
  icon: string
}>(), {
  format: 'small',
})

const size = computed(() => props.format === 'large' ? '7' : '4')
</script>

<template>
  <Icon
    :name="icon" :style="{
      width: `calc(var(--spacing) * ${size})`,
      height: `calc(var(--spacing) * ${size})`,
    }"
  />
</template>
