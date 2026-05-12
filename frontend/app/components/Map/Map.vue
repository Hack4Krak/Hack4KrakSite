<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

export type Tasks = ApiResponse<'task_list'>

const props = defineProps<{
  elements: Tasks
  completedTaskNames: string[]
}>()

const isPanelOpen = ref(true)
const flyToTarget = ref<{ lng: number, lat: number } | null>(null)
provide('flyToTarget', flyToTarget)

const availableTasks = computed<Tasks>(() => props.elements.filter(task => task.name != null) ?? [])
</script>

<template>
  <div class="flex flex-1 overflow-hidden">
    <div class="flex-1 h-screen-without-header relative">
      <ClientOnly>
        <MapView :tasks="props.elements" :completed-tasks="completedTaskNames" />
      </ClientOnly>
    </div>

    <UButton
      class="relative right-5 top-2 z-2 self-start p-2 hover:bg-muted transition-colors"
      :class="isPanelOpen ? '' : 'rotate-180'"
      aria-label="Rozwiń panel boczny"
      icon="i-weui-arrow-filled"
      @click="isPanelOpen = !isPanelOpen"
    />

    <MapSidebar :tasks="availableTasks" :completed-tasks="completedTaskNames" :is-panel-open="isPanelOpen" />
  </div>
</template>

<style scoped>
@import 'maplibre-gl/dist/maplibre-gl.css';
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
</style>
