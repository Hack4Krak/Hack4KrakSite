<script setup lang="ts">
import type { TvTask, TvTeam } from '~/composables/useTvData'

const props = defineProps<{
  teams: TvTeam[]
  tasks: TvTask[]
}>()

const scrollContainer = ref<HTMLElement | null>(null)
useAutoScroll(scrollContainer, { speed: 1, pauseMs: 3000 })

const taskProgress = computed(() => {
  const totalTeams = props.teams.length
  if (!totalTeams)
    return []

  return props.tasks.map((task) => {
    const solvers = props.teams.filter(t => t.tasks?.[task.id]).length
    return {
      id: task.id,
      name: task.name,
      solved: solvers,
      total: totalTeams,
      percent: Math.round((solvers / totalTeams) * 100),
    }
  }).sort((a, b) => b.percent - a.percent)
})
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex items-center gap-3 px-4 py-3 border-b border-surface-muted">
      <UIcon name="pixelarticons:flag" class="text-xl text-primary" />
      <h2 class="text-lg font-bold text-text-default uppercase tracking-wider">
        Zadania
      </h2>
    </div>

    <div ref="scrollContainer" class="flex-1 overflow-y-auto px-4 py-3 space-y-2.5">
      <div
        v-for="task in taskProgress"
        :key="task.id"
        class="flex items-center gap-3"
      >
        <span class="text-xs text-text-default truncate w-28 shrink-0 text-right">
          {{ task.name }}
        </span>
        <div class="flex-1 h-4 bg-surface-muted relative overflow-hidden">
          <div
            class="h-full transition-all duration-700 ease-out"
            :class="task.percent === 100 ? 'bg-primary' : 'bg-primary/60'"
            :style="{ width: `${task.percent}%` }"
          />
        </div>
        <span class="text-xs tabular-nums text-text-muted w-14 shrink-0">
          {{ task.solved }}/{{ task.total }}
        </span>
      </div>
      <div v-if="!taskProgress.length" class="text-center text-text-muted py-4">
        Brak danych
      </div>
    </div>
  </div>
</template>
