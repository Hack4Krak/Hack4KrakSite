<script setup lang="ts">
import type { EChartsOption } from 'echarts'
import * as echarts from 'echarts/core'
import { SVGRenderer } from 'echarts/renderers'

const props = withDefaults(defineProps<{
  teamName?: string
  rank?: number
  totalTeams?: number
  points?: number
  solved?: number
  totalTasks?: number
  pointsHistory?: { t: string, v: number }[]
}>(), {
  teamName: undefined,
  rank: undefined,
  totalTeams: undefined,
  points: 0,
  solved: 0,
  totalTasks: 0,
  pointsHistory: () => [],
})

echarts.use([SVGRenderer])
const initOptions = { renderer: 'svg' } as const

const chartOption = computed<EChartsOption>(() => {
  const history = props.pointsHistory
  if (!history.length)
    return { series: [] }
  const lastIndex = history.length - 1
  return {
    grid: { left: 36, right: 12, top: 10, bottom: 22, containLabel: false },
    xAxis: {
      type: 'category',
      data: history.map(p => p.t),
      boundaryGap: false,
      axisLine: { lineStyle: { color: '#262626' } },
      axisTick: { show: false },
      axisLabel: {
        color: '#808080',
        fontSize: 10,
        fontFamily: 'Pixelify Sans Hack4Krak',
        interval: Math.max(1, Math.floor(history.length / 5) - 1),
      },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisTick: { show: false },
      splitLine: { lineStyle: { color: '#262626', type: [2, 4], dashOffset: 0 } },
      axisLabel: {
        color: '#808080',
        fontSize: 10,
        fontFamily: 'Pixelify Sans Hack4Krak',
        formatter: (val: number) => (val >= 1000 ? `${val / 1000}k` : `${val}`),
      },
    },
    tooltip: {
      trigger: 'axis',
      backgroundColor: '#0a0a0a',
      borderColor: '#d08700',
      borderWidth: 1,
      textStyle: { color: '#fafafa', fontSize: 11, fontFamily: 'Pixelify Sans Hack4Krak' },
      confine: true,
      formatter: (params: any) => {
        const p = params[0]
        return `${p.axisValue} · <b style="color:#efb100">${p.value} pkt</b>`
      },
    },
    series: [
      {
        type: 'line',
        smooth: true,
        symbol: 'rect',
        symbolSize: 5,
        showSymbol: true,
        data: history.map((p, i) => ({
          value: p.v,
          symbol: i === lastIndex ? 'circle' : 'rect',
          symbolSize: i === lastIndex ? 10 : 4,
          itemStyle: i === lastIndex
            ? { color: '#efb100', borderColor: '#fafafa', borderWidth: 2 }
            : { color: '#efb100' },
        })),
        lineStyle: { color: '#efb100', width: 2 },
        animation: false,
        markLine: {
          symbol: 'none',
          silent: true,
          lineStyle: { color: '#262626', type: 'solid', width: 1 },
          data: [{ yAxis: history.at(-1)?.v ?? 0, label: { show: false } }],
        },
      },
    ],
  }
})

const stats = computed(() => [
  { label: 'Miejsce', value: props.rank ? `#${props.rank}` : '-', suffix: props.totalTeams ? `/ ${props.totalTeams}` : '' },
  { label: 'Punkty', value: props.points.toLocaleString('pl-PL'), suffix: 'pkt' },
  { label: 'Zadania', value: `${props.solved}`, suffix: `/ ${props.totalTasks}` },
])
</script>

<template>
  <section class="flex h-full min-h-0 flex-col border-2 border-surface-muted bg-default p-5 shadow-[inset_0_0_0_1px_rgb(250_250_250/0.025)] lg:p-6">
    <PanelSectionTitle class="mb-4">
      Status drużyny
    </PanelSectionTitle>

    <div class="grid shrink-0 grid-cols-3 gap-4">
      <div v-for="s in stats" :key="s.label" class="min-w-0">
        <p class="text-xs uppercase tracking-wider text-muted">
          {{ s.label }}
        </p>
        <p class="mt-1 flex items-baseline gap-1.5">
          <span class="font-pixelify text-3xl leading-none text-primary">{{ s.value }}</span>
          <span class="text-xs text-muted">{{ s.suffix }}</span>
        </p>
      </div>
    </div>

    <div class="mt-5 min-h-0 w-full flex-1">
      <VChart :option="chartOption" autoresize :init-options="initOptions" class="size-full" />
    </div>

    <NuxtLink
      to="/leaderboard"
      class="mt-4 flex shrink-0 items-center justify-between border-t-2 border-surface-muted pt-3 font-pixelify text-xs uppercase tracking-wider text-muted transition-colors hover:text-primary"
    >
      Zobacz ranking
      <UIcon name="pixelarticons:arrow-right" class="size-4" />
    </NuxtLink>
  </section>
</template>
