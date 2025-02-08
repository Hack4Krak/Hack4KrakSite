<script setup lang="ts">
import { FetchError } from 'ofetch'

const description = useState('task-description', () => '')

const route = useRoute()
const task_id = String(route.params.id ?? 'simple-task-example')

if (import.meta.server) {
  try {
    const address = '/tasks/description/{task_id}'
    const { data: response } = await useApi(address, {
      path: { task_id },
    })

    if (!response.value) {
      showError({
        statusCode: 404,
        message: 'Zadanie nie zostało znalezione',
      })
    } else {
      description.value = response.value
    }
  } catch (error) {
    if (error instanceof FetchError) {
      console.error(error.message)
    } else {
      console.error(error)
    }
  }
}
</script>

<template>
  <div class="flex flex-col mx-[10vw] w-[80vw] pt-5">
    <MarkdownContent :text="description" />
  </div>
</template>
