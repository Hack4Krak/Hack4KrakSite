<script setup lang="ts">
useOgImage()
</script>

<template>
  <main>
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" />
    <UApp>
      <Navbar />
      <slot class="min-h-[calc(100vh-var(--ui-header-height))]" />
    </UApp>
  </main>
</template>
