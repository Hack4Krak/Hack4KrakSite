<script setup lang="ts">
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

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale, TimeScale)

const { data } = await useApi('/leaderboard/chart')

const targetTimezone = 'Europe/Warsaw'

const adjustedTimestamps = computed(() => {
  return data.value?.event_timestamps.map((timeStamp: string) =>
    dayjs.utc(timeStamp).tz(targetTimezone).format(),
  ) ?? []
})

const datasets = computed(() => (data.value?.team_points_over_time || []).map(item => ({
  label: item.name,
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
</script>

<template>
  <Line :data="chartData" :options="chartOptions" />
</template>
