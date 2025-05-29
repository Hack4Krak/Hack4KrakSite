<script setup lang="ts">
import type { Tasks } from '~/components/Map.vue'
import Map from '@/components/Map.vue'

useSeoMeta({
  title: 'Zadania',
  description: 'Zobacz listę zadań na naszym CTF-ie!',
})

const { data } = await useApi('/tasks/list', { key: 'tasks-list', onResponseError: undefined })

const { data: completedTasksRaw } = await useAuth('/teams/membership/completed_tasks', {
  onResponseError: undefined,
  redirect: 'error',
})

const completedTasks = computed(() =>
  Array.isArray(completedTasksRaw.value) ? completedTasksRaw.value : [],
)

const elements = ref<Tasks>(data.value ?? [])
const taskDescriptionPopover = ref(false)

const taskIdString = ref('')
const route = useRoute()

const taskId = route.query.id
if (taskId !== undefined) {
  taskDescriptionPopover.value = true
  taskIdString.value = String(taskId)
}
</script>

<template>
  <div>
    <TaskDescription v-model:open="taskDescriptionPopover" :task-id="taskIdString" />
    <Map :elements="elements" :completed-tasks="completedTasks" class="mt-1" />
  </div>
</template>
