<script setup lang="ts">
export interface Element {
  x: number
  y: number
  content: string
}

defineProps<{
  elements: Element[]
}>()

let isDragging = false
let initialMousePositionX = 0
let scrollLeft = 0
const mapContainer = ref<HTMLElement | null>(null)

const MAP_IMAGE_ASPECT_RATIO = 2550 / 480
const IDEAL_VERTICAL_OVERFLOW_VALUE = 88

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
</script>

<template>
  <div
    ref="mapContainer"
    class="relative overflow-auto scrollbar-hide cursor-grab active:cursor-grabbing"
    @mousedown="onMouseDown"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
    @mouseleave="onMouseUp"
  >
    <div class="relative" :style="{ width: `${IDEAL_VERTICAL_OVERFLOW_VALUE * MAP_IMAGE_ASPECT_RATIO}vh` }">
      <img
        class="h-auto w-full object-cover rendering-pixelated select-none pointer-events-none"
        src="/img/map.png"
        alt="Map with tasks"
      >
      <div
        v-for="(item, index) in elements"
        :key="index"
        class="absolute transform -translate-x-1/2 -translate-y-1/2"
        :style="{ left: `${item.x}vh`, top: `${item.y}vh` }"
      >
        {{ item.content }}
      </div>
    </div>
  </div>
</template>
