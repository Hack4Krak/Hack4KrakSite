<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

export type Tasks = ApiResponse<'task_list'>

defineProps<{
  tasks: Tasks
  completedTasks: string[]
}>()

const MAP_STYLE = '/maplibre_style.json'
const MAP_CENTER: [number, number] = [19.94, 50.06]
const MAP_ZOOM = 13
const MAP_MIN_ZOOM = 12
const MAP_MAX_BOUNDS: [[number, number], [number, number]] = [[19.774, 49.965], [20.21, 50.13]]

const flyToTarget = inject('flyToTarget') as Ref<{ lng: number, lat: number } | null>
const mapInstance = useMglMap()

watch(flyToTarget, (target) => {
  if (target && mapInstance.map) {
    mapInstance.map.flyTo({ center: [target.lng, target.lat], zoom: 16 })
  }
})
</script>

<template>
  <MglMap
    :map-style="MAP_STYLE"
    :center="MAP_CENTER"
    :zoom="MAP_ZOOM"
    :min-zoom="MAP_MIN_ZOOM"
    :max-bounds="MAP_MAX_BOUNDS"
    :attribution-control="false"
  >
    <MglMarker
      v-for="task in tasks"
      :key="`${task.display.icon_coordinates.lng}-${task.display.icon_coordinates.lat}`"
      :coordinates="[task.display.icon_coordinates.lng, task.display.icon_coordinates.lat]"
    >
      <template #marker>
        <MapTaskMarker
          :task="task"
          :is-completed="completedTasks.includes(task.id)"
        />
      </template>
    </MglMarker>
  </MglMap>
</template>

<style>
// for some reason maplibre ignores tailwind classes
.maplibregl-map {
  width: 100vw !important;
}
</style>
