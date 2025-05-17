<script setup lang="ts">
const route = useRoute('tasks-description-id')
const taskId = String(route.params.id)

useSeoMeta({
  title: `Opis zadania ${taskId}`,
  description: 'Zobacz szczegóły zadania, które musisz wykonać, aby zdobyć punkty!',
})
</script>

<template>
  <TaskDescription :task-id="taskId" />
</template>
