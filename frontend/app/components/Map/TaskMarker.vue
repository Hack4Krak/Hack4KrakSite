<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import type { TaskStats } from '~/utils/taskPresentation'

export type Tasks = ApiResponse<'task_list'>

const props = defineProps<{
  task: Tasks[number]
  isCompleted: boolean
  stats?: TaskStats
}>()

const showPopup = ref(false)
const fallbackName = 'Zadanie niedostępne'

const { imgSrc, onImgError } = useTaskIcon(() => props.task.id)
const countdownText = useTaskCountdown(() => props.task.available_since)
const markerLabel = computed(() => props.task.name ?? countdownText.value ?? fallbackName)
</script>

<template>
  <div class="relative flex flex-col items-center cursor-pointer" @mouseenter="showPopup = true" @mouseleave="showPopup = false">
    <NuxtLink
      :to="{ name: 'tasks-description-id', params: { id: task.id } }"
      class="flex flex-col items-center group"
      :class="{ 'opacity-60': isCompleted }"
    >
      <span class="relative block size-12 sm:size-14">
        <span class="absolute inset-x-3 bottom-0 h-1.5 bg-black/55 blur-md" />
        <span
          v-if="isCompleted"
          class="absolute -right-1 -top-1 z-10 grid size-5 place-items-center border border-surface-muted bg-default/90 text-muted shadow-sm shadow-black/40"
          title="Ukończone"
        >
          <UIcon name="i-lucide-check" class="size-3.5" />
        </span>
        <NuxtImg
          :src="imgSrc"
          :alt="task.name ?? fallbackName"
          class="relative h-full w-full object-contain transition-transform group-hover:scale-105 rendering-pixelated drop-shadow-[0_8px_12px_rgba(0,0,0,0.55)]"
          @error="onImgError"
        />
      </span>
      <span class="-mt-0.5 max-w-32 text-center text-[10px] font-semibold leading-tight text-default drop-shadow-[0_2px_2px_rgba(0,0,0,0.9)] sm:max-w-40 sm:text-[11px]">
        {{ markerLabel }}
      </span>
    </NuxtLink>

    <div v-if="task.id" class="absolute left-1/2 top-14 z-40 -translate-x-1/2 pointer-events-none transition-all duration-150 sm:top-16">
      <MapTaskPopup
        :id="task.id"
        :name="markerLabel"
        :difficulty-estimate="task.difficulty_estimate"
        :labels="task.labels"
        :is-completed="isCompleted"
        :stats="stats"
        compact
        hide-name
        :class="showPopup ? 'mt-3 opacity-100 translate-y-0' : 'mt-1 opacity-0 -translate-y-1'"
      />
    </div>
  </div>
</template>
