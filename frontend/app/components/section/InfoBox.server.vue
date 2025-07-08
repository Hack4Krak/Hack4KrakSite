<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'
</script>

<template>
  <div class="flex flex-col gap-8 border-cyan-400 border-2 bg-cyan-400/10 p-6 text-center justify-center">
    <p class="text-xl lg:text-3xl font-bold text-cyan-400">
      {{ LANDING_CONTENT.infoBox }}
    </p>
  </div>
</template>
