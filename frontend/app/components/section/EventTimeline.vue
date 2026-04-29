<script setup lang="ts">
import type { SchemaEventStage } from '#open-fetch-schemas/api'
import { formatPolishDay, formatTime, toTimestamp } from '~/utils/date'

type TimelineState = 'current' | 'past' | 'upcoming'
interface TimelineItem {
  key: string
  label: string
  time: string
  stageType: SchemaEventStage['stage_type']
  isKeyStage: boolean
  state: TimelineState
}

const props = withDefaults(defineProps<{
  stages?: SchemaEventStage[]
}>(), {
  stages: () => [],
})

const now = useNow({ interval: 1000 })

const sortedStages = computed(() =>
  [...props.stages].sort((first, second) => first.start_date.localeCompare(second.start_date)),
)

const schedule = computed(() => {
  const nowMs = toTimestamp(now.value)

  const enriched = sortedStages.value.map((stage, index) => ({
    stage,
    key: `${stage.stage_type}-${stage.name}-${stage.start_date}`,
    end: stage.end_date
      ? toTimestamp(stage.end_date)
      : sortedStages.value[index + 1]
        ? toTimestamp(sortedStages.value[index + 1]!.start_date)
        : null,
  }))

  const activeKey = enriched.reduce<string | null>((current, { stage, key, end }) =>
    end !== null && toTimestamp(stage.start_date) <= nowMs && nowMs < end ? key : current, null)

  const groupedStages = new Map<string, { label: string, items: TimelineItem[] }>()

  for (const { stage, key, end } of enriched) {
    const dayKey = stage.start_date.slice(0, 10)
    const isPast = end !== null ? nowMs >= end : nowMs >= toTimestamp(stage.start_date)
    const state: TimelineState = activeKey === key ? 'current' : isPast ? 'past' : 'upcoming'

    if (!groupedStages.has(dayKey)) {
      groupedStages.set(dayKey, { label: formatPolishDay(stage.start_date), items: [] })
    }

    groupedStages.get(dayKey)!.items.push({
      key,
      label: stage.name,
      time: stage.end_date ? `${formatTime(stage.start_date)} - ${formatTime(stage.end_date)}` : formatTime(stage.start_date),
      stageType: stage.stage_type,
      isKeyStage: stage.stage_type !== 'informative',
      state,
    })
  }

  return Array.from(groupedStages, ([key, value]) => ({ key, ...value }))
})
</script>

<template>
  <div class="flex h-full flex-col gap-6">
    <div
      v-for="timelineDay in schedule"
      :key="timelineDay.key"
      data-timeline-day
      class="border-2 border-surface-muted"
    >
      <div class="px-5 py-3 border-b-2 border-surface-muted bg-surface-muted/30">
        <p class="font-bold text-sm uppercase tracking-wider text-primary">
          {{ timelineDay.label }}
        </p>
      </div>
      <ul class="divide-y-2 divide-surface-muted">
        <li
          v-for="item in timelineDay.items"
          :key="item.key"
          :data-stage-state="item.state"
          :data-stage-type="item.stageType"
          class="flex items-center gap-4 px-5 py-3 transition-opacity"
          :class="{
            'bg-primary/10': item.state === 'current',
            'opacity-45': item.state === 'past',
          }"
        >
          <span
            class="font-pixelify text-base w-28 flex-shrink-0 tabular-nums text-primary"
            :class="item.isKeyStage ? 'font-extrabold' : 'font-bold'"
          >
            {{ item.time }}
          </span>
          <span
            class="text-sm"
            :class="item.isKeyStage ? 'font-bold text-primary' : 'text-default'"
          >
            {{ item.label }}
          </span>
        </li>
      </ul>
    </div>
  </div>
</template>
