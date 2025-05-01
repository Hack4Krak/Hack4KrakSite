<script setup lang="ts">
const props = withDefaults(defineProps<{
  icon: string
  link: string
  size?: string
}>(), {
  size: '8',
})

const size = computed(() => `calc(var(--spacing) * ${props.size})`)
</script>

<template>
  <NuxtLink :to="link" target="_blank">
    <UIcon
      :name="icon" :style="{
        width: size,
        height: size,
      }"
    />
  </NuxtLink>
</template>
