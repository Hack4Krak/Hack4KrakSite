<script lang="ts" setup>
import type { ApiResponse } from '#open-fetch'
import type { TableColumn } from '@nuxt/ui'
import { LineChart } from 'vue-chrts'

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

const { data, refresh: refreshChart } = await useApi('/leaderboard/chart')

const targetTimezone = 'Europe/Warsaw'

const adjustedTimestamps = computed(() => {
  return data.value?.event_timestamps.map((timeStamp: string) =>
    dayjs.utc(timeStamp).tz(targetTimezone).format(),
  ) ?? []
})

const team_points_over_time = computed(() => (data.value?.team_points_over_time || []))

const chartData: any[] = []
const categories: any = {}
for (const team of team_points_over_time.value) {
  categories[team.label] = { name: team.label, color: team.color }
}

for (let i = 0; i < adjustedTimestamps.value.length; i++) {
  const obj: any = { time: adjustedTimestamps.value[i] }
  for (const team of team_points_over_time.value) {
    obj[team.label] = team.points[i]
  }
  chartData.push(obj)
}

const xFormatter = (i: number) => dayjs.utc(chartData[i].time).tz(targetTimezone).format('HH:mm')

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

function makeTooltip(values: any) {
  if (values === undefined)
    return []
  const tooltip = { ...values }
  delete tooltip.time
  const tooltipArray: [string, number][] = Object.entries(tooltip)
  return tooltipArray.sort(([_n, points1], [_n2, points2]) => points2 - points1).slice(0, 10)
}

function tooltipName(values: any) {
  if (values === undefined)
    return null
  return `Top 10 drużyn (${dayjs.utc(values.time).tz(targetTimezone).format('HH:mm')}):`
}
</script>

<template>
  <div class="my-5 text-center flex flex-col gap-5">
    <h1 class="font-bold text-5xl">
      Punktacja
    </h1>
    <div class="h-screen p-2">
      <LineChart
        :data="chartData"
        :categories="categories"
        :height="700"
        :x-formatter="xFormatter"
        x-label="Czas"
        y-label="Punkty"
        :y-grid-line="true"
        :hide-legend="false"
      >
        <template #tooltip="{ values }">
          <div class="px-4 py-2.5">
            <div
              :style="{
                color: 'var(--tooltip-value-color)',
                textTransform: 'capitalize',
                borderBottom: '1px solid rgba(255, 255, 255, 0.05)',
                marginBottom: '0.25rem',
                paddingBottom: '0.25rem',
              }"
            >
              {{ tooltipName(values) }}
            </div>
            <div
              v-for="([key, value], index) in makeTooltip(values)"
              :key="key"
              style="display: flex; align-items: center; margin-bottom: 4px"
            >
              <span
                style="width: 8px; height: 8px; border-radius: 4px; margin-right: 8px"
                :style="{
                  backgroundColor: categories[key]?.color
                    ? categories[key].color
                    : `var(--vis-color${index})`,
                }"
              />
              <div>
                <span
                  style="font-weight: 600; margin-right: 8px"
                  :style="{ color: 'var(--tooltip-label-color)' }"
                >{{ index + 1 }}. {{ key }}:</span>
                <span
                  style="font-weight: 400"
                  :style="{ color: 'var(--tooltip-value-color)' }"
                >{{ value }}</span>
              </div>
            </div>
          </div>
        </template>
      </LineChart>
    </div>
    <div class="overflow-x-auto">
      <UTable :data="teamsTableData" :columns="columns" class="flex-1 px-15" />
    </div>
  </div>
</template>
