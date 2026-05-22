<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import type { TaskStatsMap } from '~/utils/taskPresentation'
import { getTaskStats, taskDifficultyClass, taskDifficultyLabel, taskDifficultyOrder, taskLabelText } from '~/utils/taskPresentation'

export type Tasks = ApiResponse<'task_list'>
type Task = Tasks[number]

const props = defineProps<{
  tasks: Tasks
  completedTasks: string[]
  sidebarWidth: number
  taskStats: TaskStatsMap
}>()

const labelDescription = useTaskLabelDescription()

const searchQuery = ref('')
const selectedLabels = reactive(new Set<string>())
const selectedDifficulties = reactive(new Set<string>())
const sortBy = ref<'name' | 'difficulty'>('name')

const allLabels = computed(() => {
  const tags = new Set<string>()
  props.tasks?.forEach(task => task.labels?.forEach(label => tags.add(label)))
  return Array.from(tags).sort()
})

const allDifficulties = computed(() => {
  const difficulties = new Set<string>()
  props.tasks?.forEach((task) => {
    if (task.difficulty_estimate)
      difficulties.add(task.difficulty_estimate)
  })
  return Array.from(difficulties).sort(
    (a, b) => taskDifficultyOrder(a) - taskDifficultyOrder(b),
  )
})

const filteredTasks = computed<Tasks>(() => {
  let tasks = props.tasks ?? []

  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    tasks = tasks.filter((task) => {
      if (!task.name)
        return false
      return (
        task.name.toLowerCase().includes(query)
        || task.labels?.some(label => label.toLowerCase().includes(query))
      )
    })
  }

  if (selectedLabels.size > 0) {
    tasks = tasks.filter(
      task => task.name != null && task.labels?.some(label => selectedLabels.has(label)),
    )
  }

  if (selectedDifficulties.size > 0) {
    tasks = tasks.filter(task => selectedDifficulties.has(task.difficulty_estimate))
  }

  if (sortBy.value === 'name') {
    tasks = [...tasks].sort((a, b) => (a.name ?? '').localeCompare(b.name ?? ''))
  } else {
    tasks = [...tasks].sort(
      (a, b) =>
        taskDifficultyOrder(a.difficulty_estimate)
        - taskDifficultyOrder(b.difficulty_estimate),
    )
  }

  return tasks
})

const activeFilterCount = computed(
  () => selectedLabels.size + selectedDifficulties.size,
)

const hasActiveFilters = computed(
  () => activeFilterCount.value > 0 || searchQuery.value.trim() !== '',
)

function toggleSetItem(set: Set<string>, value: string) {
  set.has(value) ? set.delete(value) : set.add(value)
}

function toggleLabel(label: string) {
  toggleSetItem(selectedLabels, label)
}

function toggleDifficulty(difficulty: string) {
  toggleSetItem(selectedDifficulties, difficulty)
}

function clearAll() {
  selectedLabels.clear()
  selectedDifficulties.clear()
  searchQuery.value = ''
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

const selectedTask = ref<Task | null>(null)
const isSlideOpen = ref(false)

function openTask(task: Task) {
  selectedTask.value = task
  isSlideOpen.value = true
  goToTask(task.id)
}

const isTwoColumn = computed(() => props.sidebarWidth >= 680)
const visibleLabelLimit = computed(() => isTwoColumn.value ? 1 : 3)

const completedTaskIds = computed(() => new Set(props.completedTasks))
const completedCount = computed(
  () => props.tasks.filter(task => completedTaskIds.value.has(task.id)).length,
)

function isTaskCompleted(task: Task) {
  return completedTaskIds.value.has(task.id)
}

const sortItems = computed(() => [
  [
    {
      label: 'Alfabetycznie (A–Z)',
      icon: 'i-lucide-arrow-down-a-z',
      onSelect: () => {
        sortBy.value = 'name'
      },
    },
    {
      label: 'Wg trudności',
      icon: 'i-lucide-bar-chart-3',
      onSelect: () => {
        sortBy.value = 'difficulty'
      },
    },
  ],
])
</script>

<template>
  <div class="grid h-[calc(100vh-var(--ui-header-height))] grid-rows-[auto_minmax(0,1fr)] overflow-hidden bg-default text-default">
    <!-- Header -->
    <div class="shrink-0 border-b-2 border-surface-muted px-5 pb-4 pt-5">
      <div class="flex items-end justify-between gap-4">
        <div class="min-w-0">
          <p class="text-xs font-bold uppercase tracking-widest text-primary">
            Panel zadań
          </p>
          <h2 class="mt-1 font-pixelify text-3xl font-bold leading-none">
            Zadania
          </h2>
        </div>
        <div class="text-right">
          <p class="text-[10px] font-bold uppercase tracking-widest text-muted">
            Ukończone
          </p>
          <p class="mt-1 font-pixelify text-2xl leading-none">
            <span class="text-primary">{{ completedCount }}</span><span class="text-muted">/{{ tasks.length }}</span>
          </p>
        </div>
      </div>

      <!-- Search + sort + filter toolbar -->
      <div class="mt-4 flex items-center gap-2">
        <UInput
          v-model="searchQuery"
          icon="i-lucide-search"
          placeholder="Szukaj zadań..."
          size="sm"
          class="flex-1"
        />

        <UDropdownMenu :items="sortItems" :ui="{ content: 'min-w-56' }">
          <UTooltip text="Sortowanie">
            <button
              class="flex h-8 items-center gap-1.5 border-2 border-surface-muted px-2.5 text-muted transition-colors hover:border-primary hover:text-primary"
              :class="sortBy === 'difficulty' ? 'border-primary text-primary' : ''"
              aria-label="Sortowanie"
            >
              <UIcon
                :name="sortBy === 'difficulty' ? 'i-lucide-bar-chart-3' : 'i-lucide-arrow-down-a-z'"
                class="size-4"
              />
            </button>
          </UTooltip>
        </UDropdownMenu>

        <UPopover :content="{ align: 'end', sideOffset: 6 }">
          <button
            class="relative flex h-8 items-center gap-1.5 border-2 px-2.5 text-xs font-bold uppercase tracking-wider transition-colors"
            :class="activeFilterCount > 0 ? 'border-primary text-primary bg-primary/5' : 'border-surface-muted text-muted hover:border-primary hover:text-primary'"
            aria-label="Filtry"
          >
            <UIcon name="i-lucide-sliders-horizontal" class="size-4" />
            <span>Filtry</span>
            <span
              v-if="activeFilterCount > 0"
              class="ml-0.5 inline-flex h-4 min-w-4 items-center justify-center bg-primary px-1 text-[10px] font-bold text-default"
            >
              {{ activeFilterCount }}
            </span>
          </button>

          <template #content>
            <div class="w-72 space-y-4 border-2 border-surface-muted bg-default p-4">
              <div class="flex items-center justify-between">
                <p class="text-xs font-bold uppercase tracking-widest text-primary">
                  Filtry
                </p>
                <button
                  v-if="hasActiveFilters"
                  class="text-[10px] uppercase tracking-wider text-muted transition-colors hover:text-error"
                  @click="clearAll"
                >
                  Wyczyść
                </button>
              </div>

              <div v-if="allDifficulties.length > 0">
                <p class="mb-2 text-[10px] font-bold uppercase tracking-widest text-muted">
                  Trudność
                </p>
                <div class="flex flex-wrap gap-1.5">
                  <button
                    v-for="diff in allDifficulties"
                    :key="diff"
                    class="cursor-pointer border-2 px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider transition-colors"
                    :class="selectedDifficulties.has(diff) ? taskDifficultyClass(diff, true) : taskDifficultyClass(diff)"
                    @click="toggleDifficulty(diff)"
                  >
                    {{ taskDifficultyLabel(diff) }}
                  </button>
                </div>
              </div>

              <div v-if="allLabels.length > 0">
                <p class="mb-2 text-[10px] font-bold uppercase tracking-widest text-muted">
                  Kategorie
                </p>
                <div class="flex flex-col gap-1">
                  <button
                    v-for="tag in allLabels"
                    :key="tag"
                    class="flex w-full items-start justify-between gap-2 border-2 px-2 py-1.5 text-left transition-colors"
                    :class="selectedLabels.has(tag) ? 'border-primary bg-primary/5' : 'border-surface-muted hover:border-primary/60'"
                    @click="toggleLabel(tag)"
                  >
                    <div class="min-w-0 flex-1">
                      <p
                        class="text-[11px] font-bold uppercase tracking-wider"
                        :class="selectedLabels.has(tag) ? 'text-primary' : 'text-default'"
                      >
                        {{ taskLabelText(tag) }}
                      </p>
                      <p
                        v-if="labelDescription(tag)"
                        class="mt-0.5 text-[10px] leading-snug text-muted line-clamp-2"
                      >
                        {{ labelDescription(tag) }}
                      </p>
                    </div>
                    <UIcon
                      v-if="selectedLabels.has(tag)"
                      name="i-lucide-check"
                      class="mt-0.5 size-3.5 shrink-0 text-primary"
                    />
                  </button>
                </div>
              </div>
            </div>
          </template>
        </UPopover>
      </div>

      <!-- Active filter summary -->
      <div v-if="hasActiveFilters" class="mt-3 flex flex-wrap items-center gap-1.5">
        <span class="text-[10px] uppercase tracking-widest text-muted">
          Pokazujesz {{ filteredTasks.length }} z {{ tasks.length }}
        </span>
        <button
          class="ml-auto inline-flex items-center gap-1 text-[10px] uppercase tracking-wider text-muted transition-colors hover:text-error"
          @click="clearAll"
        >
          <UIcon name="i-lucide-x" class="size-3" />
          Wyczyść
        </button>
      </div>
    </div>

    <!-- Task list -->
    <div class="h-full overflow-y-auto scrollbar-hide">
      <div v-if="filteredTasks.length === 0" class="flex h-40 flex-col items-center justify-center gap-2 text-muted">
        <UIcon name="i-lucide-search-x" class="size-8 opacity-40" />
        <p class="text-xs uppercase tracking-[0.24em]">
          Brak zadań
        </p>
      </div>

      <div
        v-else
        :class="isTwoColumn ? 'grid grid-cols-2' : 'flex flex-col'"
      >
        <button
          v-for="task in filteredTasks"
          :key="task.id"
          class="group relative flex w-full items-center gap-4 border-b-2 border-surface-muted px-4 py-3 text-left transition-colors hover:bg-surface-muted"
          :class="[
            isTwoColumn ? 'border-r-2' : '',
            task.name == null ? 'opacity-70' : '',
            isTaskCompleted(task) ? 'opacity-55 hover:opacity-80' : '',
          ]"
          @click="openTask(task)"
        >
          <span
            v-if="task.name == null"
            aria-hidden="true"
            class="pointer-events-none absolute inset-y-0 left-0 w-0.5 bg-warning/70"
          />
          <!-- Body -->
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <UIcon
                v-if="task.name == null"
                name="i-lucide-lock"
                class="size-3.5 shrink-0 text-warning"
              />
              <UIcon
                v-else-if="isTaskCompleted(task)"
                name="i-lucide-circle-check"
                class="size-3.5 shrink-0 text-muted"
              />
              <p
                class="truncate text-sm font-bold leading-tight"
                :class="task.name == null ? 'text-muted' : 'group-hover:text-primary'"
              >
                {{ task.name ?? 'Zadanie zablokowane' }}
              </p>
            </div>
            <div class="mt-1.5 flex flex-wrap items-center gap-x-2 gap-y-1 text-[10px] uppercase tracking-wider">
              <span
                v-if="task.difficulty_estimate"
                class="border-2 px-1.5 py-0.5 font-bold"
                :class="taskDifficultyClass(task.difficulty_estimate)"
              >
                {{ taskDifficultyLabel(task.difficulty_estimate) }}
              </span>
              <UTooltip
                v-for="label in task.labels?.slice(0, visibleLabelLimit)"
                :key="label"
                :text="labelDescription(label)"
                :disabled="!labelDescription(label)"
                :delay-duration="100"
              >
                <span class="cursor-help font-bold text-muted">
                  {{ taskLabelText(label) }}
                </span>
              </UTooltip>
              <span
                v-if="task.labels && task.labels.length > visibleLabelLimit"
                class="text-muted"
              >
                +{{ task.labels.length - visibleLabelLimit }}
              </span>
            </div>
          </div>

          <!-- Right meta -->
          <div class="flex shrink-0 flex-col items-end gap-1 text-xs">
            <span class="inline-flex items-center gap-1" title="Punkty">
              <UIcon name="pixelarticons:trophy" class="size-3.5 text-primary" />
              <span class="font-bold text-default">{{ getTaskStats(taskStats, task.id).points }}</span>
            </span>
            <span class="inline-flex items-center gap-1 text-muted" title="Rozwiązania">
              <UIcon name="pixelarticons:users" class="size-3.5" />
              <span class="font-bold">{{ getTaskStats(taskStats, task.id).solveCount }}</span>
            </span>
          </div>
        </button>
      </div>
    </div>

    <USlideover
      v-model:open="isSlideOpen"
      side="right"
      :ui="{ content: 'w-[560px] max-w-full' }"
    >
      <template #content>
        <MapTaskSlide
          :task="selectedTask"
          :is-completed="selectedTask ? isTaskCompleted(selectedTask) : false"
          :stats="selectedTask ? getTaskStats(taskStats, selectedTask.id) : undefined"
          @close="isSlideOpen = false"
        />
      </template>
    </USlideover>
  </div>
</template>
