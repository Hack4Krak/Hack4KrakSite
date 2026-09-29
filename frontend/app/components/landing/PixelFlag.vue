<script setup lang="ts">
import type { V4CountryCode } from '~~/content/landing/countries'
import { FLAG_HEIGHT, FLAG_WIDTH, V4_FLAGS } from '~~/content/landing/countries'

const props = defineProps<{
  code: V4CountryCode
}>()

const rects = computed(() => V4_FLAGS[props.code])
</script>

<template>
  <svg :viewBox="`0 0 ${FLAG_WIDTH} ${FLAG_HEIGHT}`" shape-rendering="crispEdges" aria-hidden="true">
    <rect
      v-for="([x, y, width, height, fill], index) in rects"
      :key="index"
      :x="x"
      :y="y"
      :width="width"
      :height="height"
      :fill="fill"
    />
  </svg>
</template>
