<script setup lang="ts">
import { setFavicon } from '~/utils/setFavicon'

useOgImage()

onMounted(() => {
  setFavicon()
})
</script>

<template>
  <NuxtRouteAnnouncer />
  <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" />

  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
