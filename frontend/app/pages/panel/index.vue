<script setup lang="ts">
definePageMeta({
  layout: 'panel',
  middleware: ['event-registration-guard', 'event-access-guard'],
})

useSeoMeta({
  title: 'Panel CTF',
  description: 'Sprawdź swoje zadania, flagi i postępy w Hack4Krak CTF!',
})

const { data: team } = await useAuth('/teams/membership/my_team', {
  onResponseError: () => {
    throw new Error('Response error')
  },
})

const { data: teamStats, refresh: refreshTeamStats } = await useAuth('/teams/membership/stats', {
  onResponseError: () => {
    throw new Error('Response error')
  },
})

const { data: chartData, refresh: refreshChart } = await useLazyApi('/leaderboard/chart')

const rank = computed(() => teamStats.value?.rank?.[0] ?? undefined)
const totalTeams = computed(() => teamStats.value?.rank?.[1] ?? undefined)
const solved = computed(() => teamStats.value?.captured_flags ?? undefined)
const remaining = computed(() => teamStats.value?.remaining_flags ?? undefined)
const totalTasks = computed(() => {
  if (solved.value == null || remaining.value == null)
    return undefined
  return solved.value + remaining.value
})

const teamSeries = computed(() =>
  chartData.value?.team_points_over_time?.find(item => item.name === team.value?.team_name),
)

const points = computed(() => teamSeries.value?.points.at(-1) ?? 0)
const pointsHistory = computed(() => {
  const timestamps = chartData.value?.event_timestamps ?? []
  const values = teamSeries.value?.points ?? []
  return values.map((value, index) => ({
    t: new Date(timestamps[index] ?? Date.now()).toLocaleTimeString('pl-PL', { hour: '2-digit', minute: '2-digit' }),
    v: value,
  }))
})

const now = useNow({
  scheduler: callback => useIntervalFn(callback, 1000, { immediate: true }),
})
const currentHour = computed(() => now.value.getHours())
const isNight = computed(() => currentHour.value < 6 || currentHour.value >= 20)
const skylineImage = computed(() => isNight.value ? '/img/cracow_skyline_night.png' : '/img/cracow_skyline.png')

async function refreshPanelStats() {
  await Promise.allSettled([
    refreshTeamStats(),
    refreshChart(),
  ])
}
</script>

<template>
  <div class="grid h-screen-without-header w-full grid-rows-[auto_minmax(0,1fr)] gap-6 px-6 py-6 lg:gap-7 lg:px-10 lg:py-8">
    <header class="relative isolate border-b-2 border-surface-muted pb-6">
      <div class="pointer-events-none absolute inset-y-0 -right-6 z-0 w-[70%] overflow-hidden lg:-right-10" aria-hidden="true">
        <img
          :src="skylineImage"
          alt=""
          class="absolute bottom-0 right-[-8%] h-auto w-full max-w-none rendering-pixelated"
          draggable="false"
        >
        <div class="absolute inset-0 bg-[linear-gradient(to_right,var(--color-surface-default)_0%,var(--color-surface-default)_12%,rgba(17,17,17,0.85)_18%,rgba(17,17,17,0.4)_25%,rgba(17,17,17,0)_33%)]" />
      </div>

      <div class="relative z-10">
        <p class="mb-2 text-xs font-bold uppercase tracking-wider text-primary">
          Hack4Krak 2026
        </p>
        <h1 class="font-pixelify text-4xl leading-none lg:text-5xl">
          Panel CTF
        </h1>
        <div class="mt-3 flex flex-wrap items-center gap-x-4 gap-y-1 text-sm text-muted">
          <span v-if="team?.team_name">
            Grasz jako <span class="font-bold text-primary">@{{ team.team_name }}</span>
          </span>
          <span v-if="team?.team_name" class="text-surface-muted">·</span>
          <PanelTileClock />
        </div>
      </div>
    </header>

    <div class="grid min-h-0 gap-5 lg:grid-cols-[minmax(0,1.2fr)_minmax(0,1.8fr)]">
      <PanelTileTeamStatus
        class="min-h-0"
        :team-name="team?.team_name ?? undefined"
        :rank="rank"
        :total-teams="totalTeams"
        :points="points"
        :solved="solved"
        :total-tasks="totalTasks"
        :points-history="pointsHistory"
      />

      <div class="grid min-h-0 grid-rows-[auto_minmax(0,1fr)] gap-5">
        <PanelTileFlagForm @submitted="refreshPanelStats" />
        <div class="grid min-h-0 gap-5 lg:grid-cols-2">
          <PanelTileTimeline class="min-h-0" />
          <PanelTileAnnouncements class="min-h-0" />
        </div>
      </div>
    </div>
  </div>
</template>
