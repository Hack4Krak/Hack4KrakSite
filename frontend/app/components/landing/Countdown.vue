<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

withDefaults(defineProps<{
  size?: 'md' | 'xl'
}>(), {
  size: 'md',
})

const target = new Date(LANDING_CONTENT.event.startDate)
</script>

<template>
  <!-- Rendered on the server too, so the numbers are there before hydration.
       Only the minutes can differ by then, which the mismatch hint allows -->
  <Timer :target="target">
    <template #default="{ displayUnits, padded, pluralize }">
      <div class="flex" :class="size === 'xl' ? 'gap-8 sm:gap-14 lg:gap-20' : 'gap-6'">
        <div v-for="unit in displayUnits" :key="unit.key" class="flex flex-col items-center">
          <span
            class="font-pixelify leading-none font-bold tabular-nums"
            :class="size === 'xl' ? 'text-5xl lg:text-7xl' : 'text-4xl lg:text-5xl'"
            data-allow-mismatch="text"
          >
            {{ unit.key === 'days' ? unit.value : padded(unit.value) }}
          </span>
          <span
            class="mt-3 text-muted"
            :class="size === 'xl' ? 'text-base lg:text-xl' : 'text-sm'"
            data-allow-mismatch="text"
          >
            {{ pluralize(unit.value, unit.longLabel).toLowerCase() }}
          </span>
        </div>
      </div>
    </template>
  </Timer>
</template>
