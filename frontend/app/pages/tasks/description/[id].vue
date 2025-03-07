<script setup lang="ts">
const route = useRoute()
const taskId = String(route.params.id)
</script>

<template>
  <TaskDescription :task-id="taskId" />
</template>
