<script setup lang="ts">
import { FetchError } from 'ofetch'

const route = useRoute()
const taskId = String(route.params.id)

const story = ref<{ title: string, message: string }[]>([])
const backgroundImage = `${useRuntimeConfig().public.openFetch.api.baseURL}/tasks/background/${taskId}`

function checkImage() {
  const img = new Image()
  img.src = backgroundImage
  img.onerror = () => {
    showError({
      statusCode: 404,
      message: 'Zdjecie nie zostalo znalezione',
    })
  }
}

try {
  const { data: storyResponse } = await useApi('/tasks/story/{task_id}', {
    path: { task_id: taskId },
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

function redirectToTaskDescription() {
  navigateTo(`/tasks/description/${taskId}`)
}

onMounted(() => {
  checkImage()
})
</script>

<template>
  <StoryViewer :image="backgroundImage" :story-dialogues="story" @complete="redirectToTaskDescription" />
</template>
