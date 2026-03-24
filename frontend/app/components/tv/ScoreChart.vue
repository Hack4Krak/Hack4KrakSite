<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'
import * as echarts from 'echarts/core'
import { SVGRenderer } from 'echarts/renderers'

const props = defineProps<{
  chartData: ApiResponse<'chart'> | null
  startDate?: string
  endDate?: string
}>()

echarts.use([SVGRenderer])

const initOptions = { renderer: 'svg' } as const

const chartDataRef = computed(() => props.chartData)
const startDateRef = computed(() => props.startDate)
const endDateRef = computed(() => props.endDate)

const { chartOption } = useLeaderboardChart({
  chartData: chartDataRef,
  startDate: startDateRef,
  endDate: endDateRef,
  compact: true,
})
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex items-center gap-3 px-4 py-3 border-b border-surface-muted">
      <UIcon name="pixelarticons:chart" class="text-xl text-primary" />
      <h2 class="text-lg font-bold text-text-default uppercase tracking-wider">
        Postęp punktowy
      </h2>
    </div>

    <div class="flex-1 min-h-0 p-2">
      <VChart
        v-if="chartData?.team_points_over_time?.length"
        :option="chartOption"
        autoresize
        :init-options="initOptions"
        class="h-full w-full"
      />
      <div v-else class="h-full flex items-center justify-center text-text-muted">
        Oczekiwanie na dane...
      </div>
    </div>
  </div>
</template>
