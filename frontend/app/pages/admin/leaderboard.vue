<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
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
  middleware: 'admin',
  layout: 'no-navbar',
})

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

const { data, refresh: refreshChart } = await useApi('/leaderboard/chart')

const targetTimezone = 'Europe/Warsaw'

const adjustedTimestamps = computed(() => {
  return data.value?.event_timestamps.map((timeStamp: string) =>
    dayjs.utc(timeStamp).tz(targetTimezone).format(),
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

function getCurrentTime() {
  const currentDate = dayjs.utc().tz(targetTimezone)
  return currentDate.format('HH:mm:ss')
}

const currentTime = ref(getCurrentTime())

onMounted(() => {
  setInterval(() => {
    currentTime.value = getCurrentTime()
  }, 1000)
})
</script>

<template>
  <div class="flex items-center justify-center h-screen p-10 w-screen gap-8">
    <div class="border-1 border-gray-300 w-[25%] h-full flex flex-col items-center gap-5 overflow-auto">
      <div class="border-b-1 border-gray-300 h-15 flex items-center justify-between pl-5 w-full">
        <p class="font-pixelify text-xl font-bold">
          Hack4Krak - aktualny Ranking
        </p>
        <div class="h-full border-l-1 border-gray-300 flex content-center items-center">
          <Icon name="mdi:alpha-x" class="text-5xl font-bold" />
        </div>
      </div>
      <ElevatedText v-for="(team, index) in teamsTableData" :key="team.team_name" class="w-[70%]" :color-hex="colors[index % colors.length] ?? '#E6194B'">
        <div class="flex justify-between text-xl p-2">
          <div>{{ index + 1 }}. {{ team.team_name.toUpperCase() }}</div>
          <div>{{ team.current_points }}PKT</div>
        </div>
      </ElevatedText>
    </div>
    <div class="flex flex-col gap-10 w-[75%] h-full">
      <div class="border-2 border-green-500 h-[25%] w-full text-9xl text-center text-green-400 font-pixelify content-center font-bold">
        {{ currentTime }}
      </div>
      <div class="border-gray-300 border-1 h-[75%] w-full">
        <div class="border-b-1 border-gray-300 h-15 flex items-center justify-between pl-5 w-full">
          <p class="font-pixelify text-xl font-bold">
            Hack4Krak - Historia Rankingu
          </p>
          <div class="h-full border-l-1 border-gray-300 flex content-center items-center">
            <Icon name="mdi:alpha-x" class="text-5xl font-bold" />
          </div>
        </div>
        <!--        <div class="bg-blue-500 h-[calc(100%-3.75rem)] w-full" /> -->
        <Line :data="chartData" :options="chartOptions" class="m-5 max-h-[calc(100%-5.75rem)] w-full" />
      </div>
    </div>
  </div>
</template>
