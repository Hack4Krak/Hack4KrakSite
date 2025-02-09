<script setup lang="ts">
import { FetchError } from 'ofetch'

const props = defineProps<{
  taskId: string | string[] | undefined
}>()

const description = ref('')
const toast = useToast()

try {
  const task_id = String(props.taskId)
  const address = '/tasks/description/{task_id}'
  const { data: response } = await useApi(address, {
    path: { task_id },
    key: `task-description-${task_id}`,
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

  if (error.data) {
    await toast.add({ title: 'Błąd pobierania danych', description: error.data.message, color: 'error' })
  } else {
    await toast.add({ title: 'Błąd pobierania danych', color: 'error' })
  }
}
</script>

<template>
  <div class="flex flex-col mx-[10vw] w-[80vw] pt-5">
    <MarkdownContent :text="description" />
  </div>
</template>
