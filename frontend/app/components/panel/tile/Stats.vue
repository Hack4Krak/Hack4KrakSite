<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

const props = defineProps<{
  teamStats: ApiResponse<'stats'>
}>()

const stats = computed(() => [
  {
    label: 'Miejsce w rankingu',
    value: `${props.teamStats.rank?.[0]}/${props.teamStats.rank?.[1]}`,
    accent: 'text-primary',
  },
  {
    label: 'Zdobyte flagi',
    value: props.teamStats.captured_flags,
    accent: 'text-success',
  },
  {
    label: 'Pozostałe flagi',
    value: props.teamStats.remaining_flags,
    accent: 'text-default',
  },
])
</script>

<template>
  <ul class="panel-card-body grid grid-cols-1 sm:grid-cols-3 gap-3">
    <li
      v-for="item in stats"
      :key="item.label"
      class="panel-subcard text-center"
    >
      <p class="text-xs uppercase tracking-wider text-muted">
        {{ item.label }}
      </p>
      <p class="font-pixelify text-3xl lg:text-4xl mt-2 leading-none tabular-nums" :class="item.accent">
        {{ item.value }}
      </p>
    </li>
  </ul>
</template>
