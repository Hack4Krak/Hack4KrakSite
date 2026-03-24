<script setup lang="ts">
definePageMeta({
  layout: 'no-navbar',
})

useSeoMeta({
  title: 'TV – Ranking na żywo',
  robots: 'noindex',
})

const LUNCH_START = '8:10'
const LUNCH_END = '22:30'

const { teams, tasks, chartData, eventInfo, currentEvent, dismissEvent } = useTvData(10000)
const { playFirstSolve, playAllSolved } = useTvSound()

watch(currentEvent, (event) => {
  if (!event)
    return
  if (event.type === 'first-solve')
    playFirstSolve()
  else if (event.type === 'all-solved')
    playAllSolved()
})
</script>

<template>
  <div class="h-screen w-screen bg-surface-default flex flex-col overflow-hidden">
    <TvTopBar
      :start-date="eventInfo?.start_date"
      :end-date="eventInfo?.end_date"
      :lunch-start="LUNCH_START"
      :lunch-end="LUNCH_END"
    />

    <div class="flex-1 flex min-h-0">
      <div class="w-[45%] border-r border-surface-muted flex flex-col">
        <div class="flex-1 min-h-0">
          <TvLeaderboard :teams="teams" :tasks="tasks" />
        </div>
      </div>

      <div class="w-[55%] flex flex-col">
        <div class="flex-[3] min-h-0 border-b border-surface-muted">
          <TvScoreChart
            :chart-data="chartData"
            :start-date="eventInfo?.start_date"
            :end-date="eventInfo?.end_date"
          />
        </div>
        <div class="flex-[2] min-h-0">
          <TvTaskProgress :teams="teams" :tasks="tasks" />
        </div>
      </div>
    </div>

    <TvNotificationToast
      v-if="currentEvent"
      :key="`${currentEvent.type}-${currentEvent.taskName}-${currentEvent.teamName}`"
      :type="currentEvent.type"
      :task-name="currentEvent.taskName"
      :team-name="currentEvent.teamName"
      :team-color="currentEvent.teamColor"
      @dismiss="dismissEvent"
    />
  </div>
</template>
