<script setup lang="ts">
interface EventStage {
  name: string
  stage_type: 'informative' | 'event-start' | 'event-end'
  start_date: string
  end_date?: string | null
}

type TimelineState = 'current' | 'past' | 'upcoming'

const props = withDefaults(defineProps<{
  stages?: EventStage[]
  now?: string | Date
}>(), {
  stages: () => [],
  now: undefined,
})

const liveNow = useNow({ interval: 1000 })

const dayFormatter = new Intl.DateTimeFormat('pl-PL', {
  weekday: 'long',
  day: 'numeric',
  month: 'long',
})

const timeFormatter = new Intl.DateTimeFormat('pl-PL', {
  hour: '2-digit',
  minute: '2-digit',
})

function capitalize(value: string) {
  return value.charAt(0).toUpperCase() + value.slice(1)
}

function formatDay(date: string) {
  return capitalize(dayFormatter.format(new Date(date)))
}

function formatTime(date: string) {
  return timeFormatter.format(new Date(date))
}

function formatStageTime(startDate: string, endDate?: string | null) {
  const start = formatTime(startDate)
  return endDate ? `${start} - ${formatTime(endDate)}` : start
}

function stageKey(stage: EventStage) {
  return `${stage.stage_type}-${stage.name}-${stage.start_date}`
}

function toTimestamp(value: string | Date) {
  return new Date(value).getTime()
}

function resolveStageEnd(stage: EventStage, nextStage?: EventStage) {
  if (stage.end_date) {
    return toTimestamp(stage.end_date)
  }

  if (!nextStage) {
    return null
  }

  return toTimestamp(nextStage.start_date)
}

const sortedStages = computed(() =>
  [...props.stages].sort((first, second) => first.start_date.localeCompare(second.start_date)),
)

const currentTimestamp = computed(() => toTimestamp(props.now ?? liveNow.value))

const currentStageKey = computed(() => {
  let currentKey: string | null = null

  sortedStages.value.forEach((stage, index) => {
    const start = toTimestamp(stage.start_date)
    if (start > currentTimestamp.value) {
      return
    }

    const end = resolveStageEnd(stage, sortedStages.value[index + 1])
    if (end !== null && currentTimestamp.value >= end) {
      return
    }

    currentKey = stageKey(stage)
  })

  return currentKey
})

const schedule = computed(() => {
  const groupedStages = new Map<string, {
    label: string
    items: {
      key: string
      label: string
      time: string
      stageType: EventStage['stage_type']
      state: TimelineState
    }[]
  }>()

  sortedStages.value.forEach((stage, index) => {
    const dayKey = stage.start_date.slice(0, 10)
    const key = stageKey(stage)
    const end = resolveStageEnd(stage, sortedStages.value[index + 1])
    const state: TimelineState = currentStageKey.value === key
      ? 'current'
      : end !== null && currentTimestamp.value >= end
        ? 'past'
        : 'upcoming'

    if (!groupedStages.has(dayKey)) {
      groupedStages.set(dayKey, {
        label: formatDay(stage.start_date),
        items: [],
      })
    }

    groupedStages.get(dayKey)?.items.push({
      key,
      label: stage.name,
      time: formatStageTime(stage.start_date, stage.end_date),
      stageType: stage.stage_type,
      state,
    })
  })

  return Array.from(groupedStages, ([key, value]) => ({
    key,
    ...value,
  }))
})
</script>

<template>
  <div class="flex h-full flex-col gap-6">
    <div
      v-for="day in schedule"
      :key="day.key"
      data-timeline-day
      class="border-2 border-surface-muted"
    >
      <div class="px-5 py-3 border-b-2 border-surface-muted bg-surface-muted/30">
        <p class="font-bold text-sm uppercase tracking-wider text-primary">
          {{ day.label }}
        </p>
      </div>
      <ul class="divide-y-2 divide-surface-muted">
        <li
          v-for="item in day.items"
          :key="item.key"
          :data-stage-state="item.state"
          :data-stage-type="item.stageType"
          class="flex items-center gap-4 px-5 py-3 transition-opacity"
          :class="[
            item.state === 'current' ? 'bg-primary/10' : '',
            item.state === 'past' ? 'opacity-45' : '',
          ]"
        >
          <span
            class="font-pixelify text-base w-28 flex-shrink-0 tabular-nums"
            :class="item.stageType === 'informative' ? 'text-primary font-bold' : 'text-primary font-extrabold'"
          >
            {{ item.time }}
          </span>
          <span
            class="text-sm"
            :class="item.stageType === 'informative' ? 'text-default' : 'font-bold text-primary'"
          >
            {{ item.label }}
          </span>
        </li>
      </ul>
    </div>
  </div>
</template>
