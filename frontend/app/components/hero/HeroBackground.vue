<script setup lang="ts">
const { x, y } = useWindowScroll()
</script>

<template>
  <div class="hero-background -z-10 hidden lg:grid w-full place-items-center overflow-hidden">
    <div
      class="bg-[url(/img/landing_background.webp)] rendering-pixelated bg-cover w-full h-full will-change-transform"
      :style="{
        opacity: 1 - (y / 1500),
        backgroundPosition: `${x / 10}px -${y / 6}px`,
        transform: `scale(${1 + y / 10000})`,
      }"
    />
  </div>
</template>
