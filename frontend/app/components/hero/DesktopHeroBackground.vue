<script setup lang="ts">
import useSeasonalBackground from '~/composables/useSeasonalBackground'

const { y } = useWindowScroll()
const { height: windowHeight } = useWindowSize()

const background = useSeasonalBackground()

const parallaxStyle = computed(() => {
  if (!import.meta.client)
    return {}
  const maxScroll = Math.max(document.documentElement.scrollHeight - windowHeight.value, 1)
  const scrollProgress = Math.min(Math.max(y.value / maxScroll, 0), 1)
  return {
    opacity: 1 - scrollProgress,
    backgroundPosition: `0 -${scrollProgress * 150}px`,
    transform: `scale(${1 + scrollProgress * 0.15})`,
  }
})

useHead({
  link: [{
    rel: 'preload',
    as: 'image',
    href: background,
    fetchpriority: 'high',
    // Matches the `lg` breakpoint below which the background is hidden
    media: '(min-width: 64rem)',
  }],
})
</script>

<template>
  <div class="-z-10 hidden lg:grid w-full place-items-center overflow-hidden">
    <div
      class="rendering-pixelated bg-cover bg-no-repeat w-full h-full will-change-transform"
      :style="[{ backgroundImage: `url(${background})` }, parallaxStyle]"
    />
  </div>
</template>
