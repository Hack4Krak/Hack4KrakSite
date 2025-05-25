<script setup lang="ts">
import { setFavicon } from '~/utils/setFavicon'

useOgImage()

onMounted(() => {
  setFavicon()
})
</script>

<template>
  <NuxtLayout>
    <NuxtPage class="flex-grow" />
  </NuxtLayout>
</template>
