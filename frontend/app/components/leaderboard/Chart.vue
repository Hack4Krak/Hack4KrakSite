<script setup lang="ts">
import * as echarts from 'echarts/core'
import { SVGRenderer } from 'echarts/renderers'

echarts.use([SVGRenderer])

const initOptions = { renderer: 'svg' } as const

const { data: chartData } = useLazyApi('/leaderboard/chart')
const { data: eventInformation } = useLazyApi('/event/info')

const startDate = computed(() => eventInformation.value?.start_date)
const endDate = computed(() => eventInformation.value?.end_date)

const { chartOption } = useLeaderboardChart({
  chartData,
  startDate,
  endDate,
})
</script>

<template>
  <div class="h-full w-full">
    <VChart
      v-if="chartData && eventInformation"
      :option="chartOption"
      autoresize
      :init-options="initOptions"
      class="h-full w-full"
    />
  </div>
</template>
