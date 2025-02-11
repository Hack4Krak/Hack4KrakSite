<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import { useResizeObserver } from '@vueuse/core'

export type Tasks = ApiResponse<'list'>

defineProps<{
  elements: Tasks
}>()

let isDragging = false
let initialMousePositionX = 0
let scrollLeft = 0
const mapContainer = ref<HTMLElement | null>(null)
const mapImage = ref<HTMLImageElement | null>(null)

const isLoaded = ref(false)
const scaleFactor = ref(1)

const MAP_IMAGE_ASPECT_RATIO = 510 / 96
const IDEAL_VERTICAL_OVERFLOW_VALUE = 88

const taskIconBaseUrl = `${useRuntimeConfig().public.openFetch.api.baseURL}/tasks/icon/`

function onMouseDown(event: MouseEvent) {
  if (!mapContainer.value)
    return
  isDragging = true
  initialMousePositionX = event.pageX
  scrollLeft = mapContainer.value.scrollLeft
}

function onMouseMove(event: MouseEvent) {
  if (!isDragging || !mapContainer.value)
    return
  event.preventDefault()
  const currentMousePositionX = event.pageX
  const dragDistanceX = (currentMousePositionX - initialMousePositionX) * -1
  mapContainer.value.scrollLeft = scrollLeft + dragDistanceX
}

function onMouseUp() {
  isDragging = false
}

useResizeObserver(mapImage, () => {
  if (!mapImage.value)
    return
  const naturalHeight = mapImage.value.naturalHeight
  const renderedHeight = mapImage.value.clientHeight

  scaleFactor.value = renderedHeight / naturalHeight
})

onMounted(() => {
  isLoaded.value = true
})
</script>

<template>
  <div
    ref="mapContainer"
    autofocus
    class="relative overflow-auto scrollbar-hide cursor-grab active:cursor-grabbing focus:outline-none"
    @mousedown="onMouseDown"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
    @mouseleave="onMouseUp"
  >
    <div class="relative" :style="{ width: `${IDEAL_VERTICAL_OVERFLOW_VALUE * MAP_IMAGE_ASPECT_RATIO}vh` }">
      <img
        ref="mapImage"
        class="h-auto w-full object-cover rendering-pixelated select-none pointer-events-none"
        src="/img/map.png"
        alt="Map with tasks"
      >
      <div
        v-for="item in elements"
        :key="item.id"
        class="absolute transform -translate-x-1/2 -translate-y-1/2"
        :style="{ left: `${item.display.icon_coordinates.x}vh`, top: `${item.display.icon_coordinates.y}vh` }"
      >
        <div :style="{ transform: `scale(${scaleFactor})` }">
          <UTooltip :text="item.name">
            <NuxtLink :to="{ name: 'tasks-description-id', params: { id: item.id } }">
              <img v-if="isLoaded" :src="`${taskIconBaseUrl}${item.id}`" class="rendering-pixelated hover:drop-shadow-[0px_0px_2px_#555555] transition-all duration-300 ease-in-out" :alt="item.name">
            </NuxtLink>
          </UTooltip>
        </div>
      </div>
    </div>
  </div>
</template>
