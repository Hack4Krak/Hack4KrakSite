<script setup lang="ts">
const route = useRoute()
const taskId = route.params.id

const { data } = await useAuth('/tasks/solution/{task_id}', {
  path: {
    task_id: taskId?.toString() ?? '',
  },
  key: `task-solution-${taskId}`,
})
</script>

<template>
  <div class="flex flex-col mx-[10vw] w-[80vw] pt-5 gap-5">
    <MarkdownContent :text="data ?? ''" />
  </div>
</template>
