<script setup lang="ts">
interface TimerUnit {
  key: 'days' | 'hours' | 'minutes' | 'seconds'
  value: number
  longLabel: [string, string, string]
  shortLabel: string
}

const props = defineProps<{
  target: Date
}>()

const emit = defineEmits(['complete'])

const { data: currentTime, refresh } = useAsyncData('currentTimeTimer', async () => {
  const now = new Date()
  return Math.max(0, props.target.getTime() - now.getTime())
})

const { pause } = useIntervalFn(refresh, 1000, { immediate: true })

const timeLeft = computed(() => {
  const diff = currentTime.value ?? 0
  if (diff === 0) {
    pause()
    emit('complete')
  }

  return {
    seconds: Math.floor(diff / 1000) % 60,
    minutes: Math.floor(diff / (1000 * 60)) % 60,
    hours: Math.floor(diff / (1000 * 60 * 60)) % 24,
    days: Math.floor(diff / (1000 * 60 * 60 * 24)),
  }
})

function padded(number: number): string {
  return String(number).padStart(2, '0')
}

function pluralize(n: number, forms: [string, string, string]): string {
  const last = n % 10
  const lastTwo = n % 100
  if (n === 1)
    return forms[0]
  if (last >= 2 && last <= 4 && !(lastTwo >= 12 && lastTwo <= 14))
    return forms[1]
  return forms[2]
}

const allUnits = computed<TimerUnit[]>(() => [
  {
    key: 'days',
    value: timeLeft.value.days,
    longLabel: ['DZIEŃ', 'DNI', 'DNI'],
    shortLabel: 'dni',
  },
  {
    key: 'hours',
    value: timeLeft.value.hours,
    longLabel: ['GODZINA', 'GODZINY', 'GODZIN'],
    shortLabel: 'godz',
  },
  {
    key: 'minutes',
    value: timeLeft.value.minutes,
    longLabel: ['MINUTA', 'MINUTY', 'MINUT'],
    shortLabel: 'min',
  },
  {
    key: 'seconds',
    value: timeLeft.value.seconds,
    longLabel: ['SEKUNDA', 'SEKUNDY', 'SEKUND'],
    shortLabel: 'sek',
  },
])

const displayUnits = computed(() =>
  timeLeft.value.days > 0
    ? allUnits.value.filter(unit => unit.key !== 'seconds')
    : allUnits.value.filter(unit => unit.key !== 'days'),
)
</script>

<template>
  <slot :time-left="timeLeft" :padded="padded" :pluralize="pluralize" :all-units="allUnits" :display-units="displayUnits">
    <div class="flex flex-col">
      <div class="flex space-x-6 xl:text-8xl text-6xl font-semibold font-roboto">
        <template v-for="(unit, index) in displayUnits" :key="unit.key">
          <div class="flex flex-col items-center">
            <span>{{ padded(unit.value) }}</span>
            <span class="text-base mt-2">{{ pluralize(unit.value, unit.longLabel) }}</span>
          </div>
          <span v-if="index < displayUnits.length - 1">:</span>
        </template>
      </div>
    </div>
  </slot>
</template>
