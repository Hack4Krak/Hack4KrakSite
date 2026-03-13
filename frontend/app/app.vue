<script setup lang="ts">
import { setFavicon } from '~/utils/setFavicon'

useOgImage()

const isCommandPaletteOpen = useCommandPaletteState()
const isFlagModalOpen = useFlagModalState()

if (import.meta.client) {
  useKeyboardShortcuts()
}

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

    <LazyCommandPalette hydrate-on-idle v-model="isCommandPaletteOpen" />
    <LazyFlagSubmitModal hydrate-on-idle v-model="isFlagModalOpen" />
  </UApp>
</template>
