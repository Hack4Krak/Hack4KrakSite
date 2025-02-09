<script setup lang="ts">
import type { Element } from '@/components/Map.vue'
import Map from '@/components/Map.vue'

const elements = ref<Element[]>([
  { x: 10, y: 20, content: 'Placeholder' },
  { x: 30, y: 40, content: 'Placeholder' },
  { x: 50, y: 60, content: 'Placeholder' },
])
</script>

<template>
  <Map :elements="elements" class="mt-1" />
</template>
