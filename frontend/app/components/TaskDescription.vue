<script setup lang="ts">
import { FetchError } from 'ofetch'

const props = defineProps<{
  taskId: string | string[] | undefined
}>()

const description = ref('')

try {
  const taskId = String(props.taskId)
  const address = '/tasks/description/{task_id}'
  const { data: response } = await useApi(address, {
    path: { task_id: taskId },
    key: `task-description-${taskId}`,
  })

  if (response.value === undefined) {
    showError({
      statusCode: 404,
      message: 'Zadanie nie zostało znalezione',
    })
    console.error('Task not found')
  } else {
    description.value = String(response.value)
  }
} catch (error) {
  console.error(error)
  if (!(error instanceof FetchError)) {
    throw error
  }

  showError(error)
}

const { data } = await useAuth(`/tasks/assets/list/${props.taskId}`, {
  key: `task-${props.taskId}`,
})

const baseAssetsPath = `${useRuntimeConfig().public.openFetch.api.baseURL}/tasks/assets/get`

const assets = ref(data)
</script>

<template>
  <div class="flex flex-col mx-[10vw] w-[80vw] pt-5 gap-5">
    <MarkdownContent :text="description" />
    <h2 class="text-4xl font-bold">
      Assets
    </h2>
    <ul class="flex flex-col list-disc pl-5">
      <li v-for="asset in assets" :key="asset.description">
        <a :href="`${baseAssetsPath}/${taskId}/${asset.path}`" download class="w-auto text-blue-400 underline" target="_blank">
          {{ asset.description }}
        </a>
      </li>
    </ul>
  </div>
</template>
