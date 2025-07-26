<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

const props = defineProps<{
  teamStats: ApiResponse<'stats'>
}>()

const stats = computed(() => [
  {
    label: 'Miejsce w rankingu',
    value: `${props.teamStats.rank?.[0]}/${props.teamStats.rank?.[1]}`,
    class: '',
  },
  {
    label: 'Zdobyte flagi',
    value: props.teamStats.captured_flags,
    class: 'text-green-400',
  },
  {
    label: 'Pozostałe flagi',
    value: props.teamStats.remaining_flags,
    class: 'text-yellow-400',
  },
])
</script>

<template>
  <div class="flex divide-x font-pixelify text-center">
    <div
      v-for="(item, i) in stats"
      :key="i"
      class="flex-1 p-10"
    >
      <div class="text-xl">
        {{ item.label }}
      </div>
      <div class="text-4xl font-bold" :class="item.class">
        {{ item.value }}
      </div>
    </div>
  </div>
</template>
