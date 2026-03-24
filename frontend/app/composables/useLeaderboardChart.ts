import type { ApiResponse } from '#open-fetch'
import type { EChartsOption } from 'echarts'
import dayjs from 'dayjs'
import timezone from 'dayjs/plugin/timezone'
import utc from 'dayjs/plugin/utc'

dayjs.extend(utc)
dayjs.extend(timezone)

const targetTimezone = 'Europe/Warsaw'

interface TeamData {
  name: string
  points: number[]
  color: string
}

interface ChartOptions {
  chartData: Ref<ApiResponse<'chart'> | null | undefined>
  startDate: Ref<string | undefined>
  endDate: Ref<string | undefined>
  compact?: boolean
}

export function useLeaderboardChart({ chartData, startDate, endDate, compact = false }: ChartOptions) {
  const adjustedTimestamps = computed(() =>
    chartData.value?.event_timestamps?.map((ts: string) =>
      dayjs.utc(ts).tz(targetTimezone).format('YYYY-MM-DDTHH:mm:ss'),
    ) ?? [],
  )

  const chartOption = computed<EChartsOption>(() => {
    if (!chartData.value?.team_points_over_time?.length) {
      return { series: [] }
    }

    const start = startDate.value ? new Date(startDate.value) : undefined
    const end = endDate.value ? new Date(endDate.value) : undefined

    return {
      tooltip: {
        trigger: 'axis',
        order: 'valueDesc',
        confine: true,
        backgroundColor: '#171717',
        borderColor: '#262626',
        borderWidth: 1,
        textStyle: { color: '#fafafa', fontSize: compact ? 14 : 12 },
        transitionDuration: 0.3,
        axisPointer: {
          type: 'line',
          lineStyle: { color: '#737373' },
          label: {
            formatter: (params: any) =>
              dayjs(params.value).format(compact ? 'HH:mm' : 'DD.MM.YYYY HH:mm'),
          },
        },
      },

      legend: {
        type: 'scroll',
        bottom: compact ? 8 : 10,
        textStyle: { color: '#e5e5e5', fontSize: compact ? 13 : 12 },
        inactiveColor: '#525252',
      },

      grid: compact
        ? { left: 60, right: 20, top: 20, bottom: 60 }
        : { left: '3%', right: '4%', bottom: '15%' },

      xAxis: {
        type: 'time',
        min: start,
        max: end,
        ...(compact
          ? {}
          : { name: 'Czas', nameLocation: 'middle', nameGap: 35, nameTextStyle: { color: '#e5e5e5', fontSize: 12 } }),
        axisLabel: {
          formatter: compact ? '{HH}:{mm}' : '{dd}.{MM}.{yyyy} {HH}:{mm}',
          color: '#737373',
          fontSize: 12,
        },
        axisLine: { lineStyle: { color: '#262626' } },
        splitLine: { show: false },
      },

      yAxis: {
        type: 'value',
        name: compact ? 'Punkty' : 'Liczba punktów',
        nameTextStyle: { color: compact ? '#737373' : '#e5e5e5', fontSize: 12 },
        axisLabel: { color: '#737373', fontSize: 12 },
        ...(compact ? {} : { nameLocation: 'middle', nameGap: 50 }),
        splitLine: {
          lineStyle: { color: compact ? '#262626' : '#737373', width: 1, type: 'solid' },
        },
      },

      series: chartData.value.team_points_over_time.map((item: TeamData) => ({
        name: item.name,
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: compact ? 4 : 6,
        animation: compact,
        ...(compact ? { animationDuration: 500 } : {}),
        data: item.points.map((point: number, i: number) => [
          adjustedTimestamps.value[i],
          point,
        ]),
        lineStyle: {
          color: item.color,
          width: compact ? 2.5 : 2,
          ...(compact ? {} : { opacity: 0.8 }),
        },
        itemStyle: {
          color: item.color,
          borderWidth: 0,
        },
        emphasis: { disabled: true },
      })),
    }
  })

  return { chartOption }
}
