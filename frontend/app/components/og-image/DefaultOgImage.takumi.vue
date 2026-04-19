<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  title?: string
  description?: string
  icon?: string
}>(), {})

const icon = computed(() => props.icon || '/img/logo-light.png')

const siteConfig = useSiteConfig()
</script>

<template>
  <div
    class="w-full h-full flex flex-col items-center justify-between"
    style="background-color: #1e1e24; color: #ffffff; padding: 72px 60px 56px; position: relative;"
  >
    <img src="/img/landing_background.webp" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; object-fit: cover; opacity: 0.3;" alt="">
    <div class="flex flex-col items-center" style="margin-top: auto; margin-bottom: auto; z-index: 1;">
      <img v-if="icon" :src="icon" style="height: 80px; object-fit: contain;" alt="">
      <h1 style="font-size: 48px; font-weight: 700; margin: 0; margin-top: 28px; line-height: 1.2; letter-spacing: -0.5px; text-align: center; color: #ffffff;">
        {{ title || siteConfig.name }}
      </h1>
    </div>
    <p style="font-size: 16px; margin: 0; text-align: center; opacity: 0.45; line-height: 1.5; max-width: 700px; z-index: 1;">
      {{ description || siteConfig.description }}
    </p>
  </div>
</template>
