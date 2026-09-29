<script setup lang="ts">
import type { AboutUsPhoto } from '~~/content/about-us-content'

const props = defineProps<{
  kicker: string
  title: string
  description: string
  photos: AboutUsPhoto[]
}>()

// Spots around the edges of the header, in percent, leaving the middle free for the title
const SPOTS = [
  { class: 'left-[2%] top-[8%] w-[17%]', tilt: -6, drift: 60 },
  { class: 'left-[23%] top-[4%] w-[12%]', tilt: 4, drift: 90 },
  { class: 'right-[2%] top-[6%] w-[18%]', tilt: 5, drift: 50 },
  { class: 'right-[24%] top-[3%] w-[12%]', tilt: -3, drift: 80 },
  { class: 'left-[4%] bottom-[10%] w-[15%]', tilt: 3, drift: 40 },
  { class: 'right-[3%] bottom-[12%] w-[15%]', tilt: -4, drift: 70 },
  { class: 'left-[22%] bottom-[5%] w-[11%]', tilt: -2, drift: 100 },
]

const scattered = computed(() => props.photos.slice(0, SPOTS.length))
</script>

<template>
  <section class="relative isolate overflow-hidden">
    <div class="pointer-events-none absolute inset-0 hidden lg:block" aria-hidden="true">
      <PixelPhoto
        v-for="(photo, index) in scattered"
        :key="photo.src"
        :src="photo.src"
        alt=""
        :width="500"
        :tilt="SPOTS[index]!.tilt"
        class="drift absolute"
        :class="SPOTS[index]!.class"
        :style="{ '--drift': `${SPOTS[index]!.drift}px` }"
        loading="eager"
      />
    </div>

    <div class="flex gap-3 overflow-hidden px-4 pt-10 lg:hidden" aria-hidden="true">
      <PixelPhoto
        v-for="(photo, index) in scattered.slice(0, 3)"
        :key="photo.src"
        :src="photo.src"
        alt=""
        :width="400"
        :tilt="SPOTS[index]!.tilt / 2"
        class="w-1/3 shrink-0"
        loading="eager"
      />
    </div>

    <UContainer class="relative flex flex-col items-center py-16 text-center lg:min-h-[calc(100svh-var(--ui-header-height))] lg:justify-center lg:py-32">
      <p class="font-pixelify text-xl text-primary">
        {{ kicker }}
      </p>
      <h1 class="about-title mt-4 max-w-3xl font-pixelify text-5xl leading-[0.95] text-balance text-default sm:text-6xl lg:text-8xl">
        {{ title }}
      </h1>
      <p class="mt-8 max-w-xl text-base leading-relaxed text-balance text-muted lg:text-lg">
        {{ description }}
      </p>
      <UIcon name="pixelarticons:arrow-down" class="mt-12 size-8 text-primary motion-safe:animate-bob" aria-hidden="true" />
    </UContainer>
  </section>
</template>

<style scoped>
.about-title {
  text-shadow: 0.06em 0.06em 0 var(--ui-primary);
}

@supports (animation-timeline: scroll()) {
  @media (prefers-reduced-motion: no-preference) {
    .drift {
      animation: drift linear both;
      animation-timeline: scroll();
      animation-range: 0 100vh;
    }
  }
}

@keyframes drift {
  to {
    translate: 0 calc(var(--drift) * -1);
  }
}
</style>
