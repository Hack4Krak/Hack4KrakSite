<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import type { TaskStatsMap, TaskStatusMap, TaskStatusUpdate } from '~/utils/taskPresentation'

export type Tasks = ApiResponse<'task_list'>

const props = defineProps<{
  elements: Tasks
  completedTaskNames: string[]
  taskStats: TaskStatsMap
  taskStatuses: TaskStatusMap
  taskStatusUpdates: TaskStatusUpdate[]
}>()

const isPanelOpen = ref(true)
const sidebarWidth = ref(480)
const flyToTarget = ref<{ lng: number, lat: number } | null>(null)
provide('flyToTarget', flyToTarget)

const availableTasks = computed<Tasks>(() => props.elements ?? [])

const isResizing = ref(false)

function startResize(e: MouseEvent) {
  isResizing.value = true
  const startX = e.clientX
  const startWidth = sidebarWidth.value

  function onMove(e: MouseEvent) {
    const delta = startX - e.clientX
    sidebarWidth.value = Math.max(300, Math.min(900, startWidth + delta))
  }

  function onUp() {
    isResizing.value = false
    document.removeEventListener('mousemove', onMove)
    document.removeEventListener('mouseup', onUp)
  }

  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}
</script>

<template>
  <div
    class="flex h-[calc(100vh-var(--ui-header-height))] w-full overflow-hidden"
    :class="{ 'select-none': isResizing }"
  >
    <!-- Map area -->
    <div class="relative h-[calc(100vh-var(--ui-header-height))] min-w-0 flex-1 overflow-hidden">
      <ClientOnly>
        <MapView :tasks="props.elements" :completed-tasks="completedTaskNames" :task-stats="taskStats" :task-statuses="taskStatuses" />
      </ClientOnly>

      <!-- Toggle / resize handle zone -->
      <div class="absolute right-0 inset-y-0 flex flex-col items-end z-20 pointer-events-none">
        <!-- Resize handle (drag) -->
        <div
          v-if="isPanelOpen"
          class="absolute inset-y-0 right-0 w-1.5 cursor-col-resize hover:bg-primary/50 transition-colors pointer-events-auto"
          :class="isResizing ? 'bg-primary/60' : ''"
          @mousedown.prevent="startResize"
        />
        <!-- Toggle button -->
        <button
          class="pointer-events-auto mt-4 mr-0 border-2 border-r-0 border-surface-muted bg-default px-2 py-3 text-muted transition-colors hover:border-primary hover:bg-primary/5 hover:text-primary"
          :aria-label="isPanelOpen ? 'Zwiń panel boczny' : 'Rozwiń panel boczny'"
          @click="isPanelOpen = !isPanelOpen"
        >
          <UIcon
            name="i-lucide-chevrons-right"
            class="size-4 transition-transform duration-300"
            :class="isPanelOpen ? '' : 'rotate-180'"
          />
        </button>
      </div>
    </div>

    <!-- Sidebar -->
    <div
      class="h-[calc(100vh-var(--ui-header-height))] shrink-0 overflow-hidden border-l-2 border-surface-muted bg-default transition-[width] duration-300"
      :style="{ width: isPanelOpen ? `${sidebarWidth}px` : '0px' }"
    >
      <MapSidebar
        :tasks="availableTasks"
        :completed-tasks="completedTaskNames"
        :sidebar-width="sidebarWidth"
        :task-stats="taskStats"
        :task-statuses="taskStatuses"
        :task-status-updates="taskStatusUpdates"
      />
    </div>
  </div>
</template>

<style scoped>
@import 'maplibre-gl/dist/maplibre-gl.css';
</style>
