<script setup lang="ts">
const route = useRoute('tasks-description-id')
const taskId = route.params.id

const { data: taskName } = useApi('/tasks/name/{task_id}', {
  path: {
    task_id: taskId,
  },
})

useSeoMeta({
  title: `Rozwiązanie zadania ${taskName.value}`,
  description: 'Zobacz rozwiązanie zadania z naszego ostatniego CTF-u!',
})

const { data } = await useAuth('/tasks/solution/{task_id}', {
  path: {
    task_id: taskId?.toString() ?? '',
  },
})
</script>

<template>
  <div class="flex flex-col mx-[10vw] w-[80vw] pt-5 gap-5">
    <MarkdownContent :text="data ?? ''" />
  </div>
</template>
