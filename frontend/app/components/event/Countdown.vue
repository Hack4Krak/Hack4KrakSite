<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

withDefaults(defineProps<{
  size?: 'default' | 'sm'
  showFallbackDate?: boolean
}>(), {
  size: 'default',
  showFallbackDate: true,
})

const event = LANDING_CONTENT.event

const { data: eventInformation } = await useApi('/event/info', {
  key: 'event-countdown-info',
  onResponseError: undefined,
})

const [eventStart] = await useEventStartAndEnd(eventInformation)
const eventStarted = computed(() => eventStart ? Date.now() >= eventStart.getTime() : false)
</script>

<template>
  <template v-if="eventStart && !eventStarted">
    <Timer :target="eventStart">
      <template #default="{ allUnits, padded }">
        <div
          class="flex items-end justify-center"
          :class="size === 'sm' ? 'gap-4 lg:gap-5' : 'gap-5 lg:gap-6'"
        >
          <div
            v-for="unit in allUnits"
            :key="unit.key"
            class="flex flex-col items-center gap-1"
          >
            <span
              class="font-pixelify text-primary block leading-none tabular-nums"
              :class="size === 'sm'
                ? 'text-3xl lg:text-4xl font-bold w-[2.2ch] text-center'
                : 'text-4xl lg:text-5xl font-bold w-[2.6ch] text-center'"
            >
              {{ padded(unit.value) }}
            </span>
            <span
              class="text-[10px] uppercase tracking-widest text-muted block"
              :class="size === 'sm' ? 'w-[2.2ch] text-center text-[9px]' : 'w-[2.6ch] text-center'"
            >
              {{ unit.shortLabel }}
            </span>
          </div>
        </div>
      </template>
    </Timer>
  </template>

  <div
    v-else-if="eventStarted"
    class="font-pixelify text-primary text-center"
    :class="size === 'sm' ? 'text-xl' : 'text-2xl'"
  >
    <template v-if="size === 'default'">
      W trakcie! <br>
      Trzymamy za Was kciuki
    </template>
    <template v-else>
      W trakcie!
    </template>
  </div>

  <div
    v-else-if="showFallbackDate"
    class="font-pixelify text-primary text-center"
    :class="size === 'sm' ? 'text-xl' : 'text-2xl'"
  >
    {{ event.dateDisplay }}
  </div>
</template>
