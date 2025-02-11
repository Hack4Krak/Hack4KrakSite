<script setup lang="ts">
import { FetchError } from 'ofetch'

const route = useRoute()
const taskId = String(route.params.id)

const story = ref<{ title: string, message: string }[]>([])
const backgroundImage = `${useRuntimeConfig().public.openFetch.api.baseURL}/tasks/background/${taskId}`

try {
  const { data: storyResponse } = await useApi('/tasks/story/{task_id}', {
    path: { task_id: taskId },
    key: `task-story-${taskId}`,
  })

  if (storyResponse.value === undefined) {
    showError({
      statusCode: 404,
      message: 'Zadanie nie zostało znalezione',
    })
    console.error('Task not found')
  }

  if (storyResponse.value) {
    story.value = storyResponse.value ?? []
  }
} catch (error) {
  console.error(error)
  if (!(error instanceof FetchError)) {
    throw error
  }

  showError(error)
}
</script>

<template>
  <StoryViewer :image="backgroundImage" :story-dialogues="story" />
</template>
