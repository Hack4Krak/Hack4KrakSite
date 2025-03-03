<script lang="ts" setup>
import type { ChartOptions } from 'chart.js'
import {
  CategoryScale,
  Chart as ChartJS,
  Legend,
  LinearScale,
  LineElement,
  PointElement,
  TimeScale,
  Title,
  Tooltip,
} from 'chart.js'
import moment from 'moment-timezone'
import { Line } from 'vue-chartjs'

import 'chartjs-adapter-moment'

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale, TimeScale)

const colors = [
  '#E6194B',
  '#3CB44B',
  '#FFE119',
  '#4363D8',
  '#F58231',
  '#911EB4',
  '#46F0F0',
  '#F032E6',
  '#BCF60C',
  '#FABEBE',
  '#008080',
  '#E6BEFF',
  '#9A6324',
  '#FFFAC8',
  '#800000',
]

const { data } = await useApi('/leaderboard/chart', {
  key: 'leaderboard-chart',
})

const targetTimezone = 'Europe/Warsaw'

const adjustedTimestamps = computed(() => {
  return data.value?.event_timestamps.map((timeStamp: string) =>
    moment.utc(timeStamp).tz(targetTimezone).format(),
  ) ?? []
})

const chartData = ref({
  labels: adjustedTimestamps,
  datasets: (data.value?.team_points_over_time || []).map((item, index) => ({
    label: item.label,
    data: item.points,
    borderColor: colors[index % colors.length],
    lineTension: 0.2,
  })),
})

const { data: eventInformation } = await useApi('/event/info', {
  key: 'leaderboard-event-info',
})

const chartOptions = ref<ChartOptions<'line'>>({
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: {
      type: 'time',
      time: {
        unit: 'hour',
        tooltipFormat: 'yyyy-MM-DD HH:mm',
      },
      title: {
        display: true,
        text: 'Date',
      },
      min: eventInformation.value?.['start-date'],
      max: eventInformation.value?.['end-date'],
    },
    y: {
      title: {
        display: true,
        text: 'Value',
      },
    },
  },
})

const { data: teams } = await useApi('/leaderboard/teams', {
  key: 'leaderboard-teams',
})

const teamsTableData = ref(teams.value ?? [])

async function updateLeaderboard() {
  const { $api } = useNuxtApp()

  const chartDataResponse = await $api('/leaderboard/chart')
  const teamsResponse = await $api('/leaderboard/teams')

  const adjustedTimestamps = chartDataResponse.event_timestamps.map((timeStamp: string) =>
    moment.utc(timeStamp).tz(targetTimezone).format(),
  ) ?? []
  chartData.value = {
    labels: adjustedTimestamps,
    datasets: (data.value?.team_points_over_time || []).map((item, index) => ({
      label: item.label,
      data: item.points,
      borderColor: colors[index % colors.length],
      lineTension: 0.2,
    })),
  }

  teamsTableData.value = teamsResponse ?? []
}

if (import.meta.client) {
  const sseBackendAddress = `${useRuntimeConfig().public.openFetch.api.baseURL}/leaderboard/events`

  const eventSource = new EventSource(sseBackendAddress)
  eventSource.onmessage = async () => {
    await updateLeaderboard()
  }

  eventSource.onerror = (error) => {
    console.error('SSE connection error', error)
  }
}
</script>

<template>
  <div class="m-5 text-center">
    <h1 class="font-bold text-5xl">
      Punktacja
    </h1>
    <div class="h-screen">
      <Line :data="chartData" :options="chartOptions" class="m-5 " />
    </div>
    <UTable :data="teamsTableData && teamsTableData.length > 0 ? teamsTableData : [{ team_name: 'No data', current_points: 'No data', captured_flags: 'No data' }]" class="flex-1  mx-10" />
  </div>
</template>
