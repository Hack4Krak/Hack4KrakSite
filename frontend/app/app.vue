<script setup lang="ts">
useFavicon()
useOgImage()
useHead({
  meta: [
    { name: 'theme-color', content: '#FFFF00' },
  ],
})
</script>

<template>
  <NuxtLoadingIndicator color="white" />
  <Navbar />
  <UApp>
    <main>
      <NuxtLayout>
        <NuxtPage class="min-h-[calc(100vh-var(--spacing)*19)]" />
      </NuxtLayout>
    </main>
  </UApp>
</template>
