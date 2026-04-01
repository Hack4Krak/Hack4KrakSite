<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

const event = LANDING_CONTENT.event

const eventDate = new Date(`${event.dateStart}T${event.timeStart}:00`)
const now = ref(new Date())
let timer: ReturnType<typeof setInterval> | null = null

onMounted(() => {
  timer = setInterval(() => { now.value = new Date() }, 1000)
})
onUnmounted(() => { if (timer) clearInterval(timer) })

const diff = computed(() => {
  const ms = eventDate.getTime() - now.value.getTime()
  if (ms <= 0) return null
  const s = Math.floor(ms / 1000)
  return {
    days: Math.floor(s / 86400),
    hours: Math.floor((s % 86400) / 3600),
    minutes: Math.floor((s % 3600) / 60),
    seconds: s % 60,
  }
})

function pad(n: number) { return String(n).padStart(2, '0') }

const units = computed(() =>
  diff.value
    ? [
        { value: pad(diff.value.days), label: 'dni' },
        { value: pad(diff.value.hours), label: 'godz' },
        { value: pad(diff.value.minutes), label: 'min' },
        { value: pad(diff.value.seconds), label: 'sek' },
      ]
    : null,
)
</script>

<template>
  <div class="w-full border-2 border-primary overflow-hidden">
    <!-- Gold stripe across the full top -->
    <div class="h-1 w-full bg-primary" />

    <div class="grid grid-cols-1 lg:grid-cols-[1fr_auto_1fr] gap-0 divide-y-2 lg:divide-y-0 lg:divide-x-2 divide-surface-muted">

      <!-- Col 1: Date + Location -->
      <div class="flex flex-col justify-center gap-4 px-6 py-6 lg:px-8 lg:py-8">
        <p class="text-[10px] font-bold tracking-[0.3em] uppercase text-muted">
          Następna edycja
        </p>

        <div class="flex items-center gap-3">
          <UIcon name="pixelarticons:calendar" class="text-primary flex-shrink-0 size-5" />
          <span class="font-pixelify text-xl lg:text-2xl text-primary font-bold leading-tight whitespace-nowrap">
            {{ event.dateDisplay }}
          </span>
        </div>

        <div class="flex items-start gap-3">
          <UIcon name="pixelarticons:map-pin" class="text-primary flex-shrink-0 size-5 mt-0.5" />
          <div class="text-sm leading-snug">
            <p class="text-default font-semibold">
              {{ event.venue.building }}
            </p>
            <p class="text-muted">
              {{ event.venue.name }}, {{ event.venue.city }}
            </p>
          </div>
        </div>
      </div>

      <!-- Col 2: Countdown -->
      <div class="flex items-center justify-center px-6 py-6 lg:px-10 lg:py-8 bg-primary/5">
        <div v-if="units" class="flex gap-5 lg:gap-6 items-end">
          <div
            v-for="unit in units"
            :key="unit.label"
            class="flex flex-col items-center gap-1"
          >
            <span class="font-pixelify text-4xl lg:text-5xl text-primary font-bold leading-none w-[2.6ch] text-center tabular-nums">
              {{ unit.value }}
            </span>
            <span class="text-[10px] uppercase tracking-widest text-muted w-[2.6ch] text-center">{{ unit.label }}</span>
          </div>
        </div>
        <div v-else class="font-pixelify text-2xl text-primary text-center">
          Trwa!
        </div>
      </div>

      <!-- Col 3: CTA -->
      <div class="flex flex-col items-center justify-center gap-3 px-6 py-6 lg:px-8 lg:py-8">
        <NuxtLink :to="event.registrationUrl" class="w-full flex justify-center">
          <ElevatedButton class="text-base w-full max-w-[220px] justify-center">
            Zapisz się teraz
          </ElevatedButton>
        </NuxtLink>
        <div class="flex items-center gap-4 text-xs text-muted">
          <span class="flex items-center gap-1">
            <UIcon name="pixelarticons:check" class="text-primary size-3.5" />
            Bezpłatny udział
          </span>
          <span class="flex items-center gap-1">
            <UIcon name="pixelarticons:check" class="text-primary size-3.5" />
            Szkoły średnie
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
