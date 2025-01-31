<script setup lang="ts">
import Map from '@/components/Map.vue'

interface Element {
  x: number
  y: number
  content: string
}

const elements = ref<Element[]>([

])
</script>

<template>
  <Map :elements="elements" />
</template>
