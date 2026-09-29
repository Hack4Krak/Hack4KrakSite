<script setup lang="ts">
withDefaults(defineProps<{
  src: string
  alt: string
  tilt?: number
  width?: number
  ratio?: string
  loading?: 'lazy' | 'eager'
  // Opens the full, uncropped photo on click
  zoom?: boolean
}>(), {
  tilt: 0,
  width: 1000,
  ratio: '3/2',
  loading: 'lazy',
  zoom: false,
})
</script>

<template>
  <figure
    class="bg-text-default p-1.5 shadow-[6px_6px_0_0_var(--ui-primary)] sm:p-2 sm:shadow-[8px_8px_0_0_var(--ui-primary)]"
    :style="{ 'rotate': `${tilt}deg`, '--ratio': ratio }"
  >
    <!-- The crop sits in a class, not in style, so the zoomed copy that ProseImg renders shows the whole photo -->
    <ProseImg
      v-if="zoom"
      :src="src"
      :alt="alt"
      :width="width"
      :loading="loading"
      :ui="{
        base: 'block w-full rounded-none object-cover aspect-(--ratio)',
        zoomedImage: 'rounded-none',
      }"
    />
    <NuxtImg
      v-else
      :src="src"
      :alt="alt"
      :width="width"
      :loading="loading"
      class="block w-full object-cover aspect-(--ratio)"
    />
  </figure>
</template>
