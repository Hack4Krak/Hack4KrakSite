<script setup lang="ts">
const { y } = useWindowScroll()

// Fade out background as user scrolls down (like the hero section)
const bgOpacity = computed(() => {
  if (!import.meta.client) return 0.6
  const maxScroll = 500
  const progress = Math.min(Math.max(y.value / maxScroll, 0), 1)
  return 0.6 - progress * 0.5
})
</script>

<template>
  <NuxtLayout name="default">
    <div class="relative min-h-screen">
      <!-- Background with scroll-based opacity fade -->
      <div
        class="-z-10 hidden lg:block absolute inset-0 w-full h-full overflow-hidden pointer-events-none"
        aria-hidden="true"
      >
        <div
          class="bg-[url(/img/landing_background.webp)] rendering-pixelated bg-cover bg-top bg-no-repeat w-full h-full absolute inset-0"
          :style="{ opacity: bgOpacity }"
        />
        <!-- Gradient that also fades as you scroll -->
        <div
          class="absolute inset-0 bg-gradient-to-b from-default/90 via-default/70 to-default"
          :style="{ opacity: 1 - bgOpacity * 0.3 }"
        />
      </div>

      <!-- Content area with background so FAQ items aren't transparent -->
      <UContainer class="flex flex-col items-center py-12 lg:py-20 min-h-[60vh] bg-default/95">
        <slot />
      </UContainer>

      <UContainer>
        <Footer class="mt-8 lg:mt-12" />
      </UContainer>
    </div>
  </NuxtLayout>
</template>
