<script setup lang="ts">
import { defineProps, onMounted, onUnmounted, ref } from 'vue'

interface Element {
  x: number
  y: number
  content: string
}

defineProps<{
  elements: Element[]
}>()

const mapPosition = ref(0)
const isDragging = ref(false)
const startX = ref(0)
const startPosition = ref(0)

function handleKeyPress(event: KeyboardEvent) {
  if (event.key === 'd' || event.key === 'ArrowRight') {
    mapPosition.value -= 10
  } else if (event.key === 'a' || event.key === 'ArrowLeft') {
    mapPosition.value += 10
  }
  clamp()
}

function handlePointerDown(event: PointerEvent) {
  event.preventDefault()
  isDragging.value = true
  startX.value = event.pageX
  startPosition.value = mapPosition.value

  window.addEventListener('pointermove', handlePointerMove)
  window.addEventListener('pointerup', handlePointerUp)
  window.addEventListener('pointercancel', handlePointerUp)
}

function handlePointerMove(event: PointerEvent) {
  if (!isDragging.value)
    return
  event.preventDefault()
  const deltaX = event.pageX - startX.value
  mapPosition.value = startPosition.value + deltaX
  clamp()
}

function handlePointerUp() {
  isDragging.value = false
  window.removeEventListener('pointermove', handlePointerMove)
  window.removeEventListener('pointerup', handlePointerUp)
  window.removeEventListener('pointercancel', handlePointerUp)
}

function clamp() {
  if (mapPosition.value - window.innerWidth < -((88 * 5.31 / 100) * window.innerHeight)) {
    mapPosition.value = -((88 * 5.31 / 100) * window.innerHeight) + window.innerWidth
  } else if (mapPosition.value > 0) {
    mapPosition.value = 0
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyPress)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyPress)
  window.removeEventListener('pointermove', handlePointerMove)
  window.removeEventListener('pointerup', handlePointerUp)
  window.removeEventListener('pointercancel', handlePointerUp)
})
</script>

<template>
  <div class="relative overflow-hidden touch-none">
    <div
      class="relative h-[88vh] w-[calc(88vh*5.31)] cursor-grab active:cursor-grabbing select-none"
      :style="{ transform: `translateX(${mapPosition}px)` }"
      style="touch-action: none;"
      @pointerdown="handlePointerDown"
    >
      <img class="h-auto w-full object-cover rendering-pixelated select-none pointer-events-none" src="/img/map.png" alt="map">
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
