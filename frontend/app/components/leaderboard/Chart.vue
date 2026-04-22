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
const [start, end] = await useEventStartAndEnd()

const adjustedTimestamps = computed(() =>
  chartData.value?.event_timestamps?.map((ts: string) =>
    dayjs.utc(ts).tz(targetTimezone).format('YYYY-MM-DDTHH:mm:ss'),
  ) ?? [],
)

const chartOption = computed<EChartsOption>(() => {
  if (!chartData.value?.team_points_over_time?.length || !start || !end) {
    return { series: [] }
  }

  return {
    tooltip: {
      trigger: 'axis',
      order: 'valueDesc',
      confine: true,
      appendToBody: true,
      backgroundColor: '#000000',
      borderWidth: 0,
      textStyle: { color: '#ffffff' },
      transitionDuration: 1,
      renderMode: 'html',

      axisPointer: {
        type: 'line',
        label: {
          formatter: (params: any) => {
            return dayjs(params.value).format('DD.MM.YYYY HH:mm')
          },
        },
      },
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
      name: 'Data',
      nameLocation: 'middle',
      nameGap: 35,
      axisLabel: {
        formatter: '{dd}.{MM}.{yyyy} {HH}:{mm}',
      },
    },

    yAxis: {
      type: 'value',
      name: 'Liczba punktów',
      nameLocation: 'middle',
      nameGap: 50,
      splitLine: {
        lineStyle: { color: '#737373', width: 1, type: 'solid' },
      },
    },

    dataZoom: [
      {
        type: 'inside',
        xAxisIndex: 0,
        filterMode: 'none',
        minSpan: 25,
      },
      {
        type: 'inside',
        yAxisIndex: 0,
        filterMode: 'none',
        minSpan: 25,
      },
    ],

    series: chartData.value.team_points_over_time.map((item: TeamData) => ({
      name: item.name,
      type: 'line',
      smooth: true,
      symbol: 'circle',
      symbolSize: 6,
      animation: false,
      data: item.points.map((point: number, i: number) => [adjustedTimestamps.value[i], point]),
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
        disabled: true,
      },
    })),
  }
})
</script>

<template>
  <div class="h-full w-full">
    <p class="text-center text-xs text-neutral-500 mb-1 flex items-center justify-center gap-1">
      <UIcon name="pixelarticons:zap" />
      Użyj scrolla, aby przybliżyć wykres
    </p>
    <VChart v-if="chartData && start && end" :option="chartOption" autoresize class="h-full w-full" />
  </div>
</template>
