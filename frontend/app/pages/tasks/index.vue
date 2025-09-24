<script setup lang="ts">
import type { Tasks } from '~/components/Map.vue'
import Map from '@/components/Map.vue'

definePageMeta({
  middleware: [
    'event-access-guard',
  ],
})

useSeoMeta({
  title: 'Zadania',
  description: 'Zobacz listę zadań na naszym CTF-ie!',
})

const { data } = await useApi('/tasks/list')

const { data: completedTasksRaw } = await useAuth('/teams/membership/completed_tasks', {
  onResponseError: undefined,
  redirect: 'error',
})

const completedTasks = computed(() =>
  Array.isArray(completedTasksRaw.value) ? completedTasksRaw.value : [],
)

const elements = ref<Tasks>(data.value ?? [])
</script>

<template>
  <Map :elements="elements" :completed-tasks="completedTasks" />
</template>
