<script setup lang="ts" generic="T">
withDefaults(defineProps<{
  items: T[]
  duration?: number
  reverse?: boolean
}>(), {
  duration: 60,
  reverse: false,
})

defineSlots<{
  default: (props: { item: T }) => unknown
}>()
</script>

<template>
  <div class="marquee overflow-hidden py-3">
    <!-- The list is rendered twice so the loop can restart at -50% without a visible jump -->
    <div
      class="marquee-track flex w-max"
      :class="{ 'marquee-reverse': reverse }"
      :style="{ animationDuration: `${duration}s` }"
    >
      <div v-for="(item, index) in [...items, ...items]" :key="index" class="shrink-0 pr-5" :aria-hidden="index >= items.length || undefined">
        <slot :item="item" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.marquee {
  mask-image: linear-gradient(90deg, transparent, black 6%, black 94%, transparent);
}

.marquee-track {
  animation: marquee linear infinite;
}

.marquee-reverse {
  animation-direction: reverse;
}

.marquee:hover .marquee-track {
  animation-play-state: paused;
}

@keyframes marquee {
  to {
    translate: -50% 0;
  }
}

@media (prefers-reduced-motion: reduce) {
  .marquee {
    overflow-x: auto;
  }

  .marquee-track {
    animation: none;
  }
}
</style>
