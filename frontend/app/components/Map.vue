<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import maplibregl from 'maplibre-gl'
import 'maplibre-gl/dist/maplibre-gl.css'

export type Tasks = ApiResponse<'task_list'>

const props = defineProps<{
  elements: Tasks
  completedTasks: string[]
}>()

const mapContainer = ref<HTMLElement | null>(null)
const map = ref<any>(null)
const markers = ref<any[]>([])

const searchQuery = ref('')
const isPanelOpen = ref(true)
const selectedTaskId = ref<string | null>(null)

const MAP_STYLE = 'https://vector.openstreetmap.org/styles/shortbread/colorful.json'

const taskIconBaseUrl = `${useRuntimeConfig().public.openFetch.api.baseURL}/tasks/icon/`

const filteredTasks = computed<Tasks>(() => {
  if (!searchQuery.value.trim())
    return props.elements
  const query = searchQuery.value.toLowerCase()
  return props.elements.filter(task =>
    task.name.toLowerCase().includes(query)
    || task.labels.some(label => label.toLowerCase().includes(query)),
  )
})

function getCoordinates(task: Tasks[number]) {
  const coords = task.display?.icon_coordinates
  if (!coords)
    return null
  return {
    lng: coords.x,
    lat: coords.y,
  }
}

function createMarkerElement(task: Tasks[number], isCompleted: boolean) {
  const el = document.createElement('div')
  el.className = 'task-marker'
  el.innerHTML = `<img src="${taskIconBaseUrl}${task.id}" alt="${task.name}" class="w-8 h-8 cursor-pointer hover:scale-110 transition-transform" />`
  if (isCompleted) {
    el.querySelector('img')?.classList.add('drop-shadow-[0px_0px_2px_#458018]')
  } else {
    el.querySelector('img')?.classList.add('hover:drop-shadow-[0px_0px_2px_#555555]')
  }
  return el
}

function addMarkers() {
  markers.value.forEach(m => m.remove())
  markers.value = []

  filteredTasks.value.forEach((task) => {
    const coords = getCoordinates(task)
    if (!coords)
      return

    const isCompleted = props.completedTasks.includes(task.id)
    const el = createMarkerElement(task, isCompleted)

    el.addEventListener('click', () => {
      selectedTaskId.value = task.id
      navigateTo({ name: 'tasks-story-id', params: { id: task.id } })
    })

    const marker = new maplibregl.Marker({ element: el })
      .setLngLat([coords.lng, coords.lat])
      .addTo(map.value!)

    const popup = new maplibregl.Popup({ offset: 25, closeButton: false })
      .setHTML(`
    <div class="p-3 min-w-[200px] rounded-[5px] border-2 border-muted bg-accented">
      <h3 class="font-bold text-sm mb-2 text-gray-900 dark:text-white">${task.name}</h3>
      <div class="flex items-center gap-2 mb-2">
        <span class="text-xs text-gray-500 dark:text-gray-400">Trudność:</span>
        <span class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-xs text-gray-700 dark:text-gray-300">${task.difficulty_estimate}</span>
        ${isCompleted ? '<span class="px-2 py-0.5 rounded bg-green-100 dark:bg-green-900 text-xs text-green-700 dark:text-green-300">Ukończone</span>' : ''}
      </div>
      <div class="grid grid-cols-3 gap-1">
        ${task.labels.slice(0, 6).map(label => `<span class="px-1 py-0.5 rounded bg-gray-50 dark:bg-gray-800 text-[10px] text-center text-gray-600 dark:text-gray-400 truncate">${label}</span>`).join('')}
      </div>
    </div>
  `)
    marker.setPopup(popup)
    el.addEventListener('mousemove', () => {
      popup.setLngLat(marker.getLngLat())
      popup.addTo(map.value!)
    })

    el.addEventListener('mouseleave', () => {
      popup.remove()
    })

    markers.value.push(marker)
  })
}

function flyToTask(taskId: string) {
  const task = props.elements.find(t => t.id === taskId)
  if (!task)
    return
  const coords = getCoordinates(task)
  if (!coords)
    return
  map.value?.flyTo({ center: [coords.lng, coords.lat], zoom: 15 })
  selectedTaskId.value = taskId
}

onMounted(() => {
  if (!mapContainer.value)
    return

  map.value = new maplibregl.Map({
    container: mapContainer.value,
    style: MAP_STYLE as unknown as maplibregl.StyleSpecification,
    center: [19.94, 50.06],
    zoom: 12,
  })

  map.value.on('load', () => {
    addMarkers()
  })
})

watch(filteredTasks, () => {
  if (map.value)
    addMarkers()
})

onUnmounted(() => {
  markers.value.forEach(m => m.remove())
  map.value?.remove()
})
</script>

<template>
  <div class="relative w-full h-[calc(100vh-64px)]">
    <div ref="mapContainer" class="w-full h-full" />

    <div
      class="absolute top-4 right-4 z-10 flex flex-col gap-2 transition-all duration-300"
      :class="isPanelOpen ? 'w-80' : 'w-12'"
    >
      <button
        class="self-end p-2 rounded-lg bg-gray-900/90 text-white hover:bg-gray-800/90 transition-colors"
        @click="isPanelOpen = !isPanelOpen"
      >
        <UIcon v-if="isPanelOpen" name="i-mdi-chevron-right" class="w-5 h-5" />
        <UIcon v-else name="i-mdi-chevron-left" class="w-5 h-5" />
      </button>

      <div
        v-show="isPanelOpen"
        class="bg-default backdrop-blur-sm rounded-xl p-4 text-white max-h-[calc(100vh-200px)] overflow-hidden flex flex-col"
      >
        <h2 class="text-lg font-bold mb-3">
          Zadania
        </h2>

        <UInput
          v-model="searchQuery"
          icon="i-mdi-magnify"
          placeholder="Szukaj zadań..."
          class="mb-4"
          variant="outline"
        />

        <div class="flex-1 overflow-y-auto space-y-2 scrollbar-hide">
          <div
            v-for="task in filteredTasks"
            :key="task.id"
            class="p-3 rounded-lg border-2 border-muted hover:bg-gray-700 cursor-pointer transition-colors"
            :class="{ 'bg-gray-700': selectedTaskId === task.id }"
            @click="flyToTask(task.id)"
          >
            <div class="flex items-center justify-between">
              <span class="font-medium">{{ task.name }}</span>
              <UBadge v-if="completedTasks.includes(task.id)" color="success" size="xs">
                <UIcon name="i-mdi-check" class="w-3 h-3 mr-1" />
                Ukończone
              </UBadge>
            </div>
            <div class="flex items-center gap-2 mt-1">
              <UBadge size="sm" color="neutral">
                {{ task.difficulty_estimate }}
              </UBadge>
              <span v-for="label in task.labels.slice(0, 2)" :key="label" class="text-xs text-gray-400">
                {{ label }}
              </span>
            </div>
          </div>

          <div v-if="filteredTasks.length === 0" class="text-center text-gray-400 py-4">
            Brak zadań do wyświetlenia
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
.task-marker img {
  filter: drop-shadow(0px 0px 2px rgba(0, 0, 0, 0.5));
}

.maplibregl-popup-tip {
  display: none;
}

.maplibregl-popup-content {
  padding: 0;
}
</style>
