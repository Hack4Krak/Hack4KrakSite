<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import { buildTaskStats } from '~/utils/taskPresentation'

type Tasks = ApiResponse<'task_list'>

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
const { data: teams } = await useLazyApi('/leaderboard/teams_with_tasks')

const { data: completedTasksRaw } = await useAuth('/teams/membership/completed_tasks', {
  onResponseError: undefined,
  redirect: 'error',
})

const completedTaskNames = computed(() =>
  Array.isArray(completedTasksRaw.value) ? completedTasksRaw.value : [],
)

const elements = ref<Tasks>(data.value ?? [])
const taskStats = computed(() => buildTaskStats(elements.value, teams.value ?? []))
</script>

<template>
  <Map :elements="elements" :completed-task-names="completedTaskNames" :task-stats="taskStats" />
</template>
