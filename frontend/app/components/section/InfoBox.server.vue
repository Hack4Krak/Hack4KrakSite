<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'
</script>

<template>
  <div class="flex flex-col gap-8 border-green-400 border-2 bg-green-400/10 p-6 text-center justify-center">
    <p class="text-xl lg:text-3xl font-bold text-green-400">
      {{ LANDING_CONTENT.infoBox }}
    </p>
  </div>
</template>
