<script setup lang="ts">
useOgImage()
</script>

<template>
  <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" />
  <Navbar />
  <UApp>
    <main>
      <NuxtLayout>
        <NuxtPage class="min-h-[calc(100vh-var(--spacing)*19)]" />
      </NuxtLayout>
    </main>
  </UApp>
</template>
