<script setup lang="ts">
import { DialogContent, DialogOverlay, DialogPortal, DialogRoot, DialogTitle, VisuallyHidden } from 'reka-ui'

const props = defineProps<{
  taskId: string
}>()

const open = defineModel<boolean>()

const description = ref('')

const address = '/tasks/description/{task_id}'
const { data: response } = await useApi(address, {
  path: { task_id: props.taskId },
  key: `task-description-${props.taskId}`,
  onResponseError: undefined,
})

if (!response.value && props.taskId) {
  showError({
    statusCode: 404,
    message: 'Zadanie nie zostało znalezione',
  })
} else {
  description.value = String(response.value)
}

const { data: assets } = await useApi('/tasks/assets/list/{task_id}', {
  path: {
    task_id: props.taskId ?? '',
  },
  key: `task-${props.taskId}`,
  onResponseError: undefined,
})

const { error: solutionError } = await useApi('/tasks/solution/{task_id}', {
  path: {
    task_id: props.taskId,
  },
  key: `task-solution-${props.taskId}`,
  onResponseError: undefined,
})

const baseAssetsPath = `${useRuntimeConfig().public.openFetch.api.baseURL}/tasks/assets/get`
</script>

<template>
  <DialogRoot v-model:open="open" class="border-none">
    <DialogPortal>
      <DialogOverlay class="fixed inset-0 bg-black/50" />
      <DialogContent class="flex fixed top-[10vh] sm:left-[10vw] left-0 sm:w-[80vw] w-screen focus:outline-none">
        <VisuallyHidden>
          <DialogTitle>Opis zadania</DialogTitle>
        </VisuallyHidden>
        <div class="bg-[url(assets/img/scroll/scroll_left.png)] sm:w-40 w-30 bg-no-repeat bg-contain sm:h-[80vh] h-[50vh] bg-right" />
        <div class="bg-[url(assets/img/scroll/scroll_middle.png)] flex-grow w-full bg-repeat-x bg-contain sm:h-[80vh] h-[50vh]">
          <div class="sm:max-h-[50vh] max-h-[35vh] overflow-y-auto sm:my-[10vh] my-[7vh] scrollbar-scroll">
            <MarkdownContent :text="description" class="w-full h-full" />
            <h2 class="text-4xl font-bold text-black">
              Załączniki
            </h2>
            <ul class="flex flex-col list-disc pl-5">
              <li v-for="asset in assets" :key="asset.description">
                <a :href="`${baseAssetsPath}/${taskId}/${asset.path}`" download class="w-auto text-blue-700 underline" target="_blank">
                  {{ asset.description }}
                </a>
              </li>
            </ul>
            <div v-if="!solutionError" class="text-gray-900">
              <h2 class="text-4xl font-bold pb-5 text-black">
                Rozwiązanie
              </h2>
              Wydarzenie już się zakończyło! Możesz zobaczyć rozwiązanie
              <NuxtLink :to="`/tasks/solution/${taskId}`" class="link text-gray-700">
                tutaj
              </NuxtLink>
            </div>
          </div>
        </div>
        <div class="bg-[url(assets/img/scroll/scroll_right.png)] sm:w-40 w-30 bg-no-repeat bg-contain sm:h-[80vh] h-[50vh]" />
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>
