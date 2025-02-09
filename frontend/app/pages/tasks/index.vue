<script setup lang="ts">
import type { Tasks } from '~/components/Map.vue'
import Map from '@/components/Map.vue'

const { data } = await useApi('/tasks/list', {
  key: 'tasks-list',
})

const elements = ref<Tasks>(data.value ?? [])
</script>

<template>
  <Map :elements="elements" class="mt-1" />
</template>
