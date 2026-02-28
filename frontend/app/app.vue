<script setup lang="ts">
import { setFavicon } from '~/utils/setFavicon'

useOgImage()

const commandPaletteOpen = useCommandPaletteState()
useKeyboardShortcuts()

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

    <ClientOnly>
      <CommandPalette v-model="commandPaletteOpen" />
    </ClientOnly>
  </UApp>
</template>
