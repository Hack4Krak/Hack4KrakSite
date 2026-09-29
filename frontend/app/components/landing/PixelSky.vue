<script setup lang="ts">
type Sky = 'dawn' | 'day' | 'night'

const props = withDefaults(defineProps<{
  sky: Sky
  skylineScale?: number
  stars?: number
  // Overrides where the sun or moon sits, in percent of the scene
  bodyPosition?: { x: number, y: number }
}>(), {
  skylineScale: 3,
  stars: 48,
})

interface SkyLook {
  top: string
  bottom: string
  body: 'sun' | 'moon'
  x: number
  y: number
}

const SKIES: Record<Sky, SkyLook> = {
  dawn: { top: '#f59d63', bottom: '#ffdca3', body: 'sun', x: 14, y: 62 },
  day: { top: '#63bcf4', bottom: '#bfe8ff', body: 'sun', x: 42, y: 24 },
  night: { top: '#000b25', bottom: '#12306b', body: 'moon', x: 86, y: 12 },
}

const BANDS = 6

function mix(from: string, to: string, amount: number) {
  const channel = (hex: string, index: number) => Number.parseInt(hex.slice(1 + index * 2, 3 + index * 2), 16)
  return `rgb(${[0, 1, 2].map(index => Math.round(channel(from, index) + (channel(to, index) - channel(from, index)) * amount)).join(',')})`
}

// Hard colour steps instead of a smooth gradient, so the sky reads as pixel art
function bandedGradient({ top, bottom }: SkyLook) {
  const stops = Array.from({ length: BANDS }, (_, index) => {
    const color = mix(top, bottom, index / (BANDS - 1))
    return `${color} ${(index / BANDS) * 100}% ${((index + 1) / BANDS) * 100}%`
  })
  return `linear-gradient(to bottom, ${stops.join(', ')})`
}

const layers = (Object.keys(SKIES) as Sky[]).map(key => ({ key, background: bandedGradient(SKIES[key]) }))

const look = computed(() => ({ ...SKIES[props.sky], ...props.bodyPosition }))
const isNight = computed(() => props.sky === 'night')

// Deterministic pseudo-random star field, identical on server and client
const starField = computed(() => {
  let seed = 7
  const random = () => {
    seed = (seed * 16807) % 2147483647
    return seed / 2147483647
  }
  return Array.from({ length: props.stars }, (_, index) => ({
    left: random() * 100,
    top: random() * 62,
    size: random() > 0.85 ? 3 : 2,
    twinkle: index % 5 === 0,
    delay: random() * 4,
  }))
})

const skylineStyle = computed(() => ({
  height: `${45 * props.skylineScale}px`,
  backgroundSize: `${400 * props.skylineScale}px ${45 * props.skylineScale}px`,
}))
</script>

<template>
  <div class="pixel-sky pointer-events-none absolute inset-0 overflow-hidden" aria-hidden="true">
    <div
      v-for="layer in layers"
      :key="layer.key"
      class="absolute inset-0 transition-opacity duration-1000"
      :style="{ backgroundImage: layer.background, opacity: layer.key === sky ? 1 : 0 }"
    />

    <div class="absolute inset-0 transition-opacity duration-1000" :style="{ opacity: isNight ? 1 : 0 }">
      <span
        v-for="(star, index) in starField"
        :key="index"
        class="absolute bg-[#fff6d5]"
        :class="{ 'motion-safe:animate-twinkle': star.twinkle }"
        :style="{
          left: `${star.left}%`,
          top: `${star.top}%`,
          width: `${star.size}px`,
          height: `${star.size}px`,
          animationDelay: `${star.delay}s`,
        }"
      />
    </div>

    <div class="absolute inset-x-0 top-0 h-2/3 transition-opacity duration-1000" :style="{ opacity: isNight ? 0 : 1 }">
      <svg class="cloud absolute top-[14%] left-[8%] w-28 motion-safe:animate-drift" style="animation-duration: 60s" viewBox="0 0 28 8" shape-rendering="crispEdges">
        <path fill="#fff" d="M6 2h8v1h4v1h6v2h2v2H0V6h2V4h4z" />
      </svg>
      <svg class="cloud absolute top-[30%] left-[58%] w-20 motion-safe:animate-drift" style="animation-duration: 80s" viewBox="0 0 20 6" shape-rendering="crispEdges">
        <path fill="#fff" d="M5 1h7v1h3v2h3v2H0V4h2V2h3z" />
      </svg>
      <svg class="cloud absolute top-[8%] left-[78%] w-16 motion-safe:animate-drift" style="animation-duration: 70s" viewBox="0 0 16 5" shape-rendering="crispEdges">
        <path fill="#fff" d="M4 1h6v1h3v1h3v2H0V3h2V2h2z" />
      </svg>
    </div>

    <div
      class="absolute -translate-x-1/2 -translate-y-1/2 transition-all duration-1000 ease-out"
      :style="{ left: `${look.x}%`, top: `${look.y}%` }"
    >
      <svg
        class="w-12 transition-opacity duration-1000 sm:w-16"
        :style="{ opacity: look.body === 'sun' ? 1 : 0 }"
        viewBox="0 0 16 16"
        shape-rendering="crispEdges"
      >
        <path fill="#ffe27a" d="M7 0h2v2H7zM7 14h2v2H7zM0 7h2v2H0zM14 7h2v2h-2zM2 2h2v2H2zM12 2h2v2h-2zM2 12h2v2H2zM12 12h2v2h-2z" />
        <path fill="#ffc93c" d="M6 3h4v1h1v1h1v6h-1v1h-1v1H6v-1H5v-1H4V5h1V4h1z" />
        <path fill="#f39c1f" d="M11 8h1v3h-1v1h-1v1H7v-1h3v-1h1z" />
      </svg>
      <img
        src="/img/sprites/moon.png"
        alt=""
        class="absolute inset-0 m-auto w-10 rendering-pixelated transition-opacity duration-1000 sm:w-12"
        :style="{ opacity: look.body === 'moon' ? 1 : 0 }"
      >
    </div>

    <div
      class="absolute inset-x-0 bottom-0 bg-[url(/img/sprites/skyline-day.png)] bg-repeat-x bg-bottom rendering-pixelated transition-opacity duration-1000"
      :style="{ ...skylineStyle, opacity: isNight ? 0 : 1 }"
    />
    <div
      class="absolute inset-x-0 bottom-0 bg-[url(/img/sprites/skyline-night.png)] bg-repeat-x bg-bottom rendering-pixelated transition-opacity duration-1000"
      :style="{ ...skylineStyle, opacity: isNight ? 1 : 0 }"
    />
  </div>
</template>
