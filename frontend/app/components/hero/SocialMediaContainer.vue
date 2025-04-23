<script setup lang="ts">
import { LANDING_SOCIALS } from '~/content/landing-socials'
</script>

<template>
  <div class="social-media-container">
    <div class="flex gap-8 text-3xl font-bold">
      <SocialIcon
        v-for="({ name, icon, link }) in LANDING_SOCIALS"
        :key="name"
        :icon="icon"
        :link="link"
      />
    </div>
  </div>
</template>
