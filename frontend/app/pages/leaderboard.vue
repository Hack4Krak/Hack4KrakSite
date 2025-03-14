<script lang="ts" setup>
import type { ApiResponse } from '#open-fetch'
import type { TableColumn } from '@nuxt/ui'
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

export type Team = ApiResponse<'teams'>[0]

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

const { data, refresh: refreshChart } = await useApi('/leaderboard/chart', {
  key: 'leaderboard-chart',
})

const targetTimezone = 'Europe/Warsaw'

const adjustedTimestamps = computed(() => {
  return data.value?.event_timestamps.map((timeStamp: string) =>
    moment.utc(timeStamp).tz(targetTimezone).format(),
  ) ?? []
})

const datasets = computed(() => (data.value?.team_points_over_time || []).map((item, index) => ({
  label: item.label,
  data: item.points,
  borderColor: colors[index % colors.length],
  lineTension: 0.2,
})))

const chartData = ref({
  labels: adjustedTimestamps,
  datasets,
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
      min: eventInformation.value?.start_date,
      max: eventInformation.value?.end_date,
    },
    y: {
      title: {
        display: true,
        text: 'Value',
      },
    },
  },
})

const { data: teams, refresh: refreshTeams } = await useApi('/leaderboard/teams', {
  key: 'leaderboard-teams',
})

const columns: TableColumn<Team>[] = [
  {
    accessorKey: 'team_name',
  },
  {
    accessorKey: 'current_points',
  },
  {
    accessorKey: 'captured_flags',
  },
]

const teamsTableData = computed(() => teams.value ?? [])

const toast = useToast()

if (import.meta.client) {
  const sseBackendAddress = `${useRuntimeConfig().public.openFetch.api.baseURL}/leaderboard/updates`

  const eventSource = new EventSource(sseBackendAddress)
  eventSource.onmessage = async () => {
    await refreshChart()
    await refreshTeams()
  }

  eventSource.onerror = () => {
    toast.add({ title: 'Błąd połączenia', description: 'Nie można połączyć się z serwerem', color: 'error' })
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
    <UTable :data="teamsTableData" :columns="columns" class="flex-1  mx-10" />
  </div>
</template>
