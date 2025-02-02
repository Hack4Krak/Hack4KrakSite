<script setup lang="ts">
import { defineProps } from 'vue'

interface Element {
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
let map_container: HTMLElement | null = null

function onMouseDown(event: MouseEvent) {
  if (!map_container)
    return
  isDragging = true
  initialMousePositionX = event.pageX
  scrollLeft = map_container.scrollLeft
}

function onMouseMove(event: MouseEvent) {
  if (!isDragging || !map_container)
    return
  event.preventDefault()
  const currentMousePositionX = event.pageX
  const dragDistanceX = (currentMousePositionX - initialMousePositionX) * -1
  map_container.scrollLeft = scrollLeft + dragDistanceX
}

function onMouseUp() {
  isDragging = false
}

onMounted(() => {
  map_container = document.getElementById('map_container')
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
    <div class="relative h-[88vh] w-[calc(88vh*5.31)]">
      <img
        class="h-auto w-full object-cover rendering-pixelated select-none pointer-events-none"
        src="/img/map.png"
        alt="map"
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
