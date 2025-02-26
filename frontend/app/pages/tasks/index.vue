<script setup lang="ts">
import type { Tasks } from '~/components/Map.vue'
import Map from '@/components/Map.vue'

const { data } = await useApi('/tasks/list', {
  key: 'tasks-list',
})

const { data: completedTasks } = await useAuth('/teams/membership/completed_tasks', {
  key: 'teams-membership-completed-tasks',
})

const elements = ref<Tasks>(data.value ?? [])
</script>

<template>
  <Map :elements="elements" :completed-tasks="completedTasks ?? []" class="mt-1" />
</template>
