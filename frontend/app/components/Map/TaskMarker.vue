<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

export type Tasks = ApiResponse<'task_list'>

const props = defineProps<{
  task: Tasks[number]
  isCompleted: boolean
}>()

const taskIconBaseUrl = `${useRuntimeConfig().public.openFetch.api.baseURL}/tasks/icon/`

const showPopup = ref(false)
const iconErrored = ref(false)

const imgSrc = computed(() => {
  if (iconErrored.value || !props.task.id)
    return '/img/task_unavailable.png'
  return taskIconBaseUrl + props.task.id
})

function onImgError() {
  iconErrored.value = true
}

const now = useTimestamp({ interval: 1000 })

const countdownText = computed(() => {
  if (!props.task.available_since)
    return null
  const diff = new Date(props.task.available_since).getTime() - now.value
  if (diff <= 0)
    return null
  return humanizeDifference(diff)
})
</script>

<template>
  <div class="flex flex-col items-center gap-1 cursor-pointer" @mouseenter="showPopup = true" @mouseleave="showPopup = false">
    <NuxtLink
      :to="{ name: 'tasks-description-id', params: { id: task.id } }"
      class="flex flex-col items-center gap-1 group"
      :class="{ 'opacity-60': isCompleted }"
    >
      <NuxtImg
        :src="imgSrc"
        :alt="task.name ?? 'Cóż to może być???'"
        class="w-8 h-8 transition-transform"
        @error="onImgError"
      />
      <span class="text-xs text-center text-toned">
        {{ task.name ?? countdownText }}
      </span>
    </NuxtLink>

    <div v-if="task.id" class="absolute z-40 pointer-events-none">
      <MapTaskPopup
        :id="task.id"
        :name="task.name"
        :difficulty-estimate="task.difficulty_estimate"
        :labels="task.labels"
        :is-completed="isCompleted"
        :class="showPopup ? 'mt-10 opacity-100' : 'mt-5 opacity-0'"
      />
    </div>
  </div>
</template>
