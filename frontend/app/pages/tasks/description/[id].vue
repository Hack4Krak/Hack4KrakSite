<script setup lang="ts">
useSeoMeta({
  title: 'Opis zadania',
})

const route = useRoute('tasks-description-id')
const taskId = String(route.params.id)
</script>

<template>
  <TaskDescription :task-id="taskId" />
</template>
