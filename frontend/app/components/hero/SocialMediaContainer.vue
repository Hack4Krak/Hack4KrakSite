<script setup lang="ts">
import { LANDING_SOCIALS } from '~~/content/landing-socials'
</script>

<template>
  <div class="flex gap-8 text-3xl font-bold">
    <SocialIcon
      v-for="({ label, icon, to }) in LANDING_SOCIALS"
      :key="label"
      :aria-label="label"
      :icon="icon"
      :to="to"
    />
  </div>
</template>
