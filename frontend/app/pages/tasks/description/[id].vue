<script setup lang="ts">
const route = useRoute('tasks-description-id')
const taskId = String(route.params.id)

const { data: taskName } = useApi('/tasks/name/{task_id}', {
  path: {
    task_id: taskId,
  },
})

useSeoMeta({
  title: `Opis zadania ${taskName.value}`,
  description: 'Zobacz szczegóły zadania, które musisz wykonać, aby zdobyć punkty!',
})
</script>

<template>
  <TaskDescription :task-id="taskId" />
</template>
