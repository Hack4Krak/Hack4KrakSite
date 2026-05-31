<script setup lang="ts">
const { y } = useWindowScroll()
const { height: windowHeight } = useWindowSize()

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
    href: '/img/landing_background.webp',
  }],
})
</script>

<template>
  <div class="-z-10 hidden lg:grid w-full place-items-center overflow-hidden">
    <div
      class="bg-[url(/img/landing_background.webp)] rendering-pixelated bg-cover bg-no-repeat w-full h-full will-change-transform"
      :style="parallaxStyle"
    />
  </div>
</template>
