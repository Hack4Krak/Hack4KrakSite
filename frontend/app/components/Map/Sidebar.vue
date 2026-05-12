<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

export type Tasks = ApiResponse<'task_list'>

const props = defineProps<{
  tasks: Tasks
  completedTasks: string[]
  isPanelOpen: boolean
}>()

const searchQuery = ref('')
const selectedLabels = reactive(new Set())
const selectedDifficulties = reactive(new Set())

const allLabels = computed(() => {
  const tags = new Set<string>()
  const elements = props.tasks ?? []
  elements.forEach((task) => {
    const labels = task.labels ?? []
    labels.forEach(label => tags.add(label))
  })
  return Array.from(tags).sort()
})

const allDifficulties = computed(() => {
  const difficulties = new Set<string>()
  const elements = props.tasks ?? []
  elements.forEach((task) => {
    if (task.difficulty_estimate) {
      difficulties.add(task.difficulty_estimate)
    }
  })
  return Array.from(difficulties).sort()
})

const filteredTasks = computed<Tasks>(() => {
  let tasks = props.tasks ?? []

  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    tasks = tasks
      .filter((task) => {
        if (!task.name) {
          return false
        }
        const matchesName = task.name.toLowerCase().includes(query)
        const labels = task.labels
        const matchesLabel = Array.isArray(labels) && labels.some(label => label.toLowerCase().includes(query))
        return matchesName || matchesLabel
      })
  }

  if (selectedLabels.size > 0) {
    tasks = tasks
      .filter(task => task.name != null)
      .filter((task) => {
        const labels = task.labels ?? []
        return Array.isArray(labels) && labels.some(label => selectedLabels.has(label))
      })
  }

  if (selectedDifficulties.size > 0) {
    tasks = tasks.filter(task => selectedDifficulties.has(task.difficulty_estimate))
  }

  return tasks
})

function toggleLabel(label: string) {
  if (selectedLabels.has(label)) {
    selectedLabels.delete(label)
  } else {
    selectedLabels.add(label)
  }
}

function clearLabels() {
  selectedLabels.clear()
}

function toggleDifficulty(difficulty: string) {
  if (selectedDifficulties.has(difficulty)) {
    selectedDifficulties.delete(difficulty)
  } else {
    selectedDifficulties.add(difficulty)
  }
}

const flyToTarget = inject('flyToTarget') as Ref<{ lng: number, lat: number } | null>

function goToTask(id: string) {
  const task = props.tasks?.find(t => t.id === id)
  if (flyToTarget && task?.display?.icon_coordinates) {
    flyToTarget.value = {
      lng: task.display.icon_coordinates.lng,
      lat: task.display.icon_coordinates.lat,
    }
  }
}
</script>

<template>
  <div class="fixed right-0 bg-default w-80 transition-all duration-300 overflow-scroll" :class="{ '-mr-80': !isPanelOpen }">
    <div class="p-4 border-b border-border flex items-center justify-between">
      <h2 class="font-bold text-lg">
        Zadania
      </h2>
    </div>

    <div class="p-4 space-y-4 flex-1 flex flex-col overflow-hidden">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Szukaj zadań..."
        class="w-full px-3 py-2 border border-border rounded-md bg-background text-foreground placeholder-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary"
      >

      <div v-if="allLabels.length > 0" class="mb-4">
        <div class="flex items-center justify-between mb-2">
          <span class="text-xs text-toned">Tagi</span>
          <UButton
            v-if="selectedLabels.size > 0"
            @click="clearLabels"
          >
            Wyczyść
          </UButton>
        </div>
        <div class="flex flex-wrap gap-1 max-h-24 overflow-y-auto">
          <UButton
            v-for="tag in allLabels"
            :key="tag"
            class="px-2 py-0.5 text-xs rounded transition-colors cursor-pointer"
            :class="!selectedLabels.has(tag) ? 'bg-gray-700 hover:bg-gray-600 text-gray-300' : ''"
            @click="toggleLabel(tag)"
          >
            {{ tag }}
          </UButton>
        </div>
      </div>

      <div v-if="allDifficulties.length > 0" class="mb-4">
        <div class="flex items-center justify-between mb-2">
          <span class="text-xs text-toned">Trudność</span>
          <UButton
            v-if="selectedDifficulties.size > 0"
            @click="selectedDifficulties.clear()"
          >
            Wyczyść
          </UButton>
        </div>
        <div class="flex flex-wrap gap-1">
          <UButton
            v-for="difficulty in allDifficulties"
            :key="difficulty"
            class="px-2 py-0.5 text-xs rounded transition-colors cursor-pointer"
            :class="!selectedDifficulties.has(difficulty) ? 'bg-gray-700 hover:bg-gray-600 text-gray-300' : ''"
            @click="toggleDifficulty(difficulty)"
          >
            {{ difficulty }}
          </UButton>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto space-y-2 scrollbar-hide">
        <MapTaskPopup
          v-for="task in filteredTasks"
          :id="task.id"
          :key="task.id"
          :name="task.name"
          :difficulty-estimate="task.difficulty_estimate"
          :labels="task.labels"
          :is-completed="completedTasks.includes(task.id)"
          class="block"
          @click="goToTask(task.id)"
        />

        <div v-if="filteredTasks.length === 0" class="flex items-center justify-center h-32 text-muted-foreground">
          <p class="text-sm">
            Brak zadań
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
</style>
