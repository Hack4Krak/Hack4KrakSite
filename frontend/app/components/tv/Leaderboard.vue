<script setup lang="ts">
import type { TvTask, TvTeam } from '~/composables/useTvData'
import dayjs from 'dayjs'

const props = defineProps<{
  teams: TvTeam[]
  tasks: TvTask[]
}>()

const scrollContainer = ref<HTMLElement | null>(null)
useAutoScroll(scrollContainer, { speed: 1, pauseMs: 3000 })

const podiumConfig = [
  { height: 'h-28', barColor: 'bg-gray-400', glowColor: 'shadow-[0_0_20px_rgba(156,163,175,0.3)]', label: '2', iconColor: 'text-gray-300', icon: 'lucide:medal', isFirst: false },
  { height: 'h-36', barColor: 'bg-yellow-400', glowColor: 'shadow-[0_0_30px_rgba(250,204,21,0.4)]', label: '1', iconColor: 'text-yellow-400', icon: 'pixelarticons:trophy', isFirst: true },
  { height: 'h-22', barColor: 'bg-amber-600', glowColor: 'shadow-[0_0_15px_rgba(217,119,6,0.25)]', label: '3', iconColor: 'text-amber-600', icon: 'lucide:medal', isFirst: false },
]

const podiumEntries = computed(() => {
  if (props.teams.length < 3)
    return []
  const ordered = [props.teams[1], props.teams[0], props.teams[2]]
  return ordered.map((team, i) => ({
    team: team!,
    config: podiumConfig[i]!,
  }))
})

const restTeams = computed(() => props.teams.slice(3))

function getTasksSolved(team: TvTeam) {
  if (!team.tasks)
    return 0
  return Object.keys(team.tasks).length
}

function getLastSolveTime(team: TvTeam) {
  if (!team.tasks)
    return null
  const times = Object.values(team.tasks)
  if (!times.length)
    return null
  const sorted = [...times].sort((a, b) => new Date(b).getTime() - new Date(a).getTime())
  return dayjs(sorted[0]).format('HH:mm')
}
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex items-center gap-3 px-4 py-3 border-b border-surface-muted">
      <UIcon name="pixelarticons:trophy" class="text-xl text-primary" />
      <h2 class="text-lg font-bold text-text-default uppercase tracking-wider">
        Ranking
      </h2>
    </div>

    <div v-if="podiumEntries.length === 3" class="px-4 pt-5 pb-2">
      <div class="flex items-end justify-center gap-3">
        <div
          v-for="entry in podiumEntries"
          :key="entry.team.team_id"
          class="flex-1 flex flex-col items-center"
        >
          <div class="flex flex-col items-center gap-1.5 mb-3">
            <UIcon
              :name="entry.config.icon"
              :class="[entry.config.iconColor, entry.config.isFirst ? 'text-4xl podium-bounce' : 'text-2xl']"
            />
            <span
              class="font-bold text-center truncate max-w-full px-1" :class="[
                entry.config.isFirst ? 'text-base text-yellow-400' : 'text-sm text-text-default',
              ]"
            >
              {{ entry.team.team_name }}
            </span>
            <span
              class="font-bold tabular-nums font-pixelify" :class="[
                entry.config.isFirst ? 'text-3xl text-yellow-400 podium-glow-text' : 'text-xl text-primary',
              ]"
            >
              {{ entry.team.current_points ?? 0 }}
            </span>
            <span class="text-xs text-text-muted tabular-nums">
              {{ getTasksSolved(entry.team) }}/{{ tasks.length }} flag
            </span>
          </div>

          <div
            class="w-full flex items-center justify-center transition-all duration-700" :class="[
              entry.config.height,
              entry.config.barColor,
              entry.config.glowColor,
            ]"
          >
            <span class="text-4xl font-bold font-pixelify text-surface-default/80">
              {{ entry.config.label }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="teams.length > 0 && teams.length < 3" class="px-4 pt-4">
      <div
        v-for="(team, index) in teams"
        :key="team.team_id"
        class="flex items-center gap-3 py-2 px-3 mb-1 border border-surface-muted"
      >
        <span class="text-lg font-bold tabular-nums text-primary w-8 text-center">{{ index + 1 }}</span>
        <div class="w-3 h-3 shrink-0" :style="{ backgroundColor: team.color || '#737373' }" />
        <span class="font-semibold text-text-default flex-1 truncate">{{ team.team_name }}</span>
        <span class="font-bold text-primary tabular-nums">{{ team.current_points ?? 0 }} pkt</span>
      </div>
    </div>

    <div ref="scrollContainer" class="flex-1 overflow-hidden border-t border-surface-muted mt-2">
      <table v-if="restTeams.length" class="w-full">
        <tbody>
          <tr
            v-for="(team, index) in restTeams"
            :key="team.team_id"
            class="border-b border-surface-muted/50 transition-all duration-500 hover:bg-surface-muted/30"
          >
            <td class="py-2 px-4 text-center w-10">
              <span class="text-text-muted text-sm tabular-nums">{{ index + 4 }}</span>
            </td>
            <td class="py-2 px-4">
              <div class="flex items-center gap-2">
                <div class="w-2.5 h-2.5 shrink-0" :style="{ backgroundColor: team.color || '#737373' }" />
                <span class="text-text-default truncate text-sm">{{ team.team_name }}</span>
              </div>
            </td>
            <td class="py-2 px-4 text-center w-20">
              <span class="font-bold text-primary tabular-nums">{{ team.current_points ?? 0 }}</span>
            </td>
            <td class="py-2 px-4 text-center w-16">
              <span class="text-text-muted tabular-nums text-xs">
                {{ getTasksSolved(team) }}/{{ tasks.length }}
              </span>
            </td>
            <td class="py-2 px-4 text-center w-16">
              <span class="text-text-muted text-xs tabular-nums">
                {{ getLastSolveTime(team) ?? '—' }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
      <div v-if="!teams.length" class="flex items-center justify-center h-full text-text-muted">
        Brak danych
      </div>
    </div>
  </div>
</template>
