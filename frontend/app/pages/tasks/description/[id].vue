<script setup lang="ts">
const route = useRoute()
const task_id = route.params.id
</script>

<template>
  <TaskDescription :task-id="task_id" />
</template>
