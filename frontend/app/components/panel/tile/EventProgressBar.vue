<script setup lang="ts">
import useEventState from '~/composables/useEventState'

const { data: timeLeft, refresh: updateTimeLeft } = useAsyncData('timeLeft', async () => useEventState())

useRafFn(() => updateTimeLeft())
</script>

<template>
  <div
    class="col-span-2 justify-center items-center flex-col flex font-pixelify py-3"
    :style="{
      background: `linear-gradient(to right, ${timeLeft?.color} ${timeLeft?.percentage}%, transparent ${timeLeft?.percentage}%)`,
    }"
  >
    <span class="text-5xl">{{ timeLeft?.diff }}</span>
    <span v-if="!timeLeft?.hidePercentage">{{ timeLeft?.percentage }}%</span>
  </div>
</template>
