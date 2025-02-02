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
let mapContainer: HTMLElement | null = null
const mapImageAspectRatio = 5.31
const idealVerticalOverflowValue = 88

function onMouseDown(event: MouseEvent) {
  if (!mapContainer)
    return
  isDragging = true
  initialMousePositionX = event.pageX
  scrollLeft = mapContainer.scrollLeft
}
function onMouseMove(event: MouseEvent) {
  if (!isDragging || !mapContainer)
    return
  event.preventDefault()
  const currentMousePositionX = event.pageX
  const dragDistanceX = (currentMousePositionX - initialMousePositionX) * -1
  mapContainer.scrollLeft = scrollLeft + dragDistanceX
}

function onMouseUp() {
  isDragging = false
}

onMounted(() => {
  mapContainer = document.getElementById('map_container')
})
</script>

<template>
  <div
    id="map_container"
    class="relative overflow-auto scrollbar-hide cursor-grab active:cursor-grabbing select-none"
    @mousedown="onMouseDown"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
    @mouseleave="onMouseUp"
  >
    <div class="relative" :style="{ width: `${idealVerticalOverflowValue * mapImageAspectRatio}vh` }">
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
