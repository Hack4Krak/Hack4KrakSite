<script setup lang="ts">
const { y } = useWindowScroll()

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
      :style="{
        opacity: 1 - (y / 1500),
        backgroundPosition: `0 -${y / 6}px`,
        transform: `scale(${1 + y / 10000})`,
      }"
    />
  </div>
</template>
