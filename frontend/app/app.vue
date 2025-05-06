<script setup lang="ts">
import { setFavicon } from '~/utils/setFavicon'

useOgImage()

onMounted(() => {
  setFavicon()
})
</script>

<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
