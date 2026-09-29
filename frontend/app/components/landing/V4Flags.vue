<script setup lang="ts">
import { V4_COUNTRIES } from '~~/content/landing/countries'
</script>

<template>
  <!-- The same row of V4 flags on the landing page and in the story on "O nas" -->
  <ul class="flex flex-wrap gap-x-6 gap-y-3">
    <li v-for="country in V4_COUNTRIES" :key="country.code" class="flex items-center gap-2">
      <LandingPixelFlag :code="country.code" class="w-8 border-2 border-surface-muted" />
      <span class="font-pixelify text-default">{{ country.name }}</span>
    </li>
  </ul>
</template>
