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

import { Line } from 'vue-chartjs'
import 'chartjs-adapter-dayjs-4/dist/chartjs-adapter-dayjs-4.esm'

definePageMeta({
  middleware: [
    'event-access-guard',
  ],
})

useSeoMeta({
  title: 'Ranking',
  description: 'Zobacz aktualny ranking drużyn i ich punkty w czasie rzeczywistym!',
})

export type Team = ApiResponse<'teams'>[0]

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale, TimeScale)

const { data, refresh: refreshChart } = await useApi('/leaderboard/chart')

const targetTimezone = 'Europe/Warsaw'

const adjustedTimestamps = computed(() => {
  return data.value?.event_timestamps.map((timeStamp: string) =>
    dayjs.utc(timeStamp).tz(targetTimezone).format(),
  ) ?? []
})

const datasets = computed(() => (data.value?.team_points_over_time || []).map(item => ({
  label: item.label,
  data: item.points,
  borderColor: item.color,
  lineTension: 0.2,
})))

const chartData = ref({
  labels: adjustedTimestamps,
  datasets,
})

const { data: eventInformation } = await useApi('/event/info')

const chartOptions = ref<ChartOptions<'line'>>({
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: {
      type: 'time',
      time: {
        unit: 'hour',
        tooltipFormat: 'DD.MM.YYYY HH:mm',
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

const { data: teams, refresh: refreshTeams } = await useApi('/leaderboard/teams')

const columns: TableColumn<Team>[] = [
  {
    accessorKey: 'team_name',
    header: 'Nazwa drużyny',
  },
  {
    accessorKey: 'current_points',
    header: 'Ilosć punktów',
  },
  {
    accessorKey: 'captured_flags',
    header: 'Zdobyte flagi',
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
  <div class="my-5 text-center flex flex-col gap-5">
    <h1 class="font-bold text-5xl">
      Punktacja
    </h1>
    <div class="overflow-x-auto">
      <div class="h-screen min-w-[50rem] px-10">
        <Line :data="chartData" :options="chartOptions" />
      </div>
    </div>
    <div class="overflow-x-auto">
      <UTable :data="teamsTableData" :columns="columns" class="flex-1 px-15" />
    </div>
  </div>
</template>
