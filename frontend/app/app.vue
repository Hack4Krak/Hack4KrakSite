<script setup lang="ts">
setFavicon()
useOgImage()
</script>

<template>
  <NuxtLoadingIndicator color="white" />
  <Navbar />
  <main>
    <NuxtLayout>
      <NuxtPage class="min-h-[calc(100vh-var(--spacing)*20)]" />
    </NuxtLayout>
  </main>
</template>
