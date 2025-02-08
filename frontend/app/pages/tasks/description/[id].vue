<script setup lang="ts">
const route = useRoute()
const task_id = route.params.id ?? undefined
</script>

<template>
  <TaskDescription :task-id="task_id" />
</template>
