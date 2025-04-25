<script setup lang="ts">
import { useFavicon, usePreferredDark } from '@vueuse/core'

// Using link tags with `prefers-color-scheme`, svg icons, and more do not work!
// The best solution so far is to implement custom JavaScript logic (for example GitHub does that too).
//
// Big thanks to @Lucky_luke for implementation:
// https://stackoverflow.com/questions/77135452/how-to-change-the-favicon-dynamically-in-nuxt-3
function setFavicon() {
  const isDark = usePreferredDark()
  const favicon = computed(() => isDark.value ? '/favicon-dark.ico' : '/favicon-light.ico')

  useFavicon(favicon, {
    rel: 'icon',
  })
}

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
