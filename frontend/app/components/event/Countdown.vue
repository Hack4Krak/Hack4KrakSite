<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

withDefaults(defineProps<{
  size?: 'lg' | 'md' | 'sm'
  showFallbackDate?: boolean
}>(), {
  size: 'md',
  showFallbackDate: true,
})

const event = LANDING_CONTENT.event

const { data: eventInformation } = await useApi('/event/info', {
  onResponseError: undefined,
})

const [eventStart] = await useEventStartAndEnd(eventInformation)
const now = ref(Date.now())
const eventStarted = computed(() => eventStart ? now.value >= eventStart.getTime() : false)
</script>

<template>
  <template v-if="eventStart && !eventStarted">
    <Timer :target="eventStart" @complete="now = Date.now()">
      <template #default="{ allUnits, padded }">
        <div
          class="grid grid-cols-4 text-center"
          :class="{
            'gap-3': size === 'sm',
            'gap-4 lg:gap-5': size === 'md',
            'gap-5 lg:gap-6': size === 'lg',
          }"
        >
          <div
            v-for="unit in allUnits"
            :key="unit.key"
            class="flex flex-col items-center"
          >
            <span
              class="font-pixelify text-primary block leading-none tabular-nums font-bold"
              :class="{
                'text-2xl': size === 'sm',
                'text-3xl lg:text-4xl': size === 'md',
                'text-4xl lg:text-5xl': size === 'lg',
              }"
            >
              {{ padded(unit.value) }}
            </span>
            <span
              class="mt-1 block text-xs uppercase tracking-widest text-muted"
              :class="{
                'scale-90': size === 'sm',
              }"
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
    :class="{
      'text-xl': size === 'sm',
      'text-2xl': size === 'md' || size === 'lg',
    }"
  >
    <template v-if="size === 'lg'">
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
    :class="{
      'text-xl': size === 'sm',
      'text-2xl': size === 'md' || size === 'lg',
    }"
  >
    {{ event.dateDisplay }}
  </div>
</template>
