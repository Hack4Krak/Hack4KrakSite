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

    description.value = response.value ?? 'No task here'
    if (description.value === 'No task here') {
      showError({
        statusCode: 404,
        message: 'Task not found',
      })
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
    <div class="prose prose-invert">
      <MDC :value="description" />
    </div>
  </div>
</template>
