<script setup lang="ts">
import type { EChartsOption } from 'echarts'
import dayjs from 'dayjs'
import timezone from 'dayjs/plugin/timezone'
import utc from 'dayjs/plugin/utc'

dayjs.extend(utc)
dayjs.extend(timezone)

interface TeamData {
  name: string
  points: number[]
  color: string
}

const targetTimezone = 'Europe/Warsaw'

const { data: chartData } = useLazyApi('/leaderboard/chart')
const { data: eventInformation } = useLazyApi('/event/info')

const adjustedTimestamps = computed(() =>
  chartData.value?.event_timestamps?.map((ts: string) =>
    dayjs.utc(ts).tz(targetTimezone).format('YYYY-MM-DDTHH:mm:ss'),
  ) ?? [],
)

const chartOption = computed<EChartsOption>(() => {
  if (!chartData.value?.team_points_over_time?.length || !eventInformation.value) {
    return { series: [] }
  }

  const { start_date, end_date } = eventInformation.value
  const start = start_date ? new Date(start_date) : undefined
  const end = end_date ? new Date(end_date) : undefined

  return {
    tooltip: {
      trigger: 'axis',
      order: 'valueDesc',
      confine: true,
      appendToBody: true,
      backgroundColor: '#000000',
      borderWidth: 0,
      textStyle: { color: '#ffffff' },
      axisPointer: { type: 'cross' },
      transitionDuration: 0,
      renderMode: 'html',
    },

    legend: {
      type: 'scroll',
      bottom: 10,
      textStyle: { color: '#e5e5e5', fontSize: 12 },
      inactiveColor: '#525252',
      animation: true,
      animationDurationUpdate: 300,
    },

    grid: {
      left: '3%',
      right: '4%',
      bottom: '15%',
    },

    xAxis: {
      type: 'time',
      min: start,
      max: end,
      name: 'Date',
      nameLocation: 'middle',
      nameGap: 35,
      axisLabel: {
        formatter: '{dd}.{MM}.{yyyy} {HH}:{mm}',
      },
    },

    yAxis: {
      type: 'value',
      name: 'Value',
      nameLocation: 'middle',
      nameGap: 50,
      splitLine: {
        lineStyle: { color: '#737373', width: 1, type: 'solid' },
      },
    },

    series: chartData.value.team_points_over_time.map((item: TeamData) => ({
      name: item.name,
      type: 'line',
      smooth: true,
      symbol: 'circle',
      symbolSize: 6,
      animation: false,
      data: item.points.map((point: number, i: number) => [
        adjustedTimestamps.value[i],
        point,
      ]),
      lineStyle: {
        color: item.color,
        width: 2,
        opacity: 0.8,
      },
      itemStyle: {
        color: item.color,
        borderWidth: 0,
      },
      emphasis: {
        disabled: false,
        focus: 'series',
        scale: false,
      },
    })),
  }
})
</script>

<template>
  <div class="h-full w-full">
    <VChart
      v-if="chartData && eventInformation"
      :option="chartOption"
      autoresize
      class="h-full w-full"
    />
  </div>
</template>
