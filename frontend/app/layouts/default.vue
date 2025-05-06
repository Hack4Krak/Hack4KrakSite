<script setup lang="ts">
import { setFavicon } from '~/utils/setFavicon'

useOgImage()

onMounted(() => {
  setFavicon()
})
</script>

<template>
  <main>
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" class="hidden" />
    <UApp>
      <Navbar />
      <slot />
    </UApp>
  </main>
</template>
