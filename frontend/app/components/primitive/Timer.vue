<script setup lang="ts">
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
</script>

<template>
  <div class="flex flex-col">
    <div class="flex space-x-6 xl:text-8xl text-6xl font-semibold font-roboto">
      <template v-if="timeLeft.days > 0">
        <div class="flex flex-col items-center">
          <span>{{ padded(timeLeft.days) }}</span>
          <span class="text-base mt-2">{{ pluralize(timeLeft.days, ['DZIEŃ', 'DNI', 'DNI']) }}</span>
        </div>
        <span>:</span>
        <div class="flex flex-col items-center">
          <span>{{ padded(timeLeft.hours) }}</span>
          <span class="text-base mt-2">{{ pluralize(timeLeft.hours, ['GODZINA', 'GODZINY', 'GODZIN']) }}</span>
        </div>
        <span>:</span>
        <div class="flex flex-col items-center">
          <span>{{ padded(timeLeft.minutes) }}</span>
          <span class="text-base mt-2">{{ pluralize(timeLeft.minutes, ['MINUTA', 'MINUTY', 'MINUT']) }}</span>
        </div>
      </template>

      <template v-else>
        <div class="flex flex-col items-center">
          <span>{{ padded(timeLeft.hours) }}</span>
          <span class="text-base mt-2">{{ pluralize(timeLeft.hours, ['GODZINA', 'GODZINY', 'GODZIN']) }}</span>
        </div>
        <span>:</span>
        <div class="flex flex-col items-center">
          <span>{{ padded(timeLeft.minutes) }}</span>
          <span class="text-base mt-2">{{ pluralize(timeLeft.minutes, ['MINUTA', 'MINUTY', 'MINUT']) }}</span>
        </div>
        <span>:</span>
        <div class="flex flex-col items-center">
          <span>{{ padded(timeLeft.seconds) }}</span>
          <span class="text-base mt-2">{{ pluralize(timeLeft.seconds, ['SEKUNDA', 'SEKUNDY', 'SEKUND']) }}</span>
        </div>
      </template>
    </div>
  </div>
</template>
