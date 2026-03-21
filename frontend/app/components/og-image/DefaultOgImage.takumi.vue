<script setup lang="ts">
/**
 * @credits Nuxt SEO <https://nuxtseo.com/>
 */

import { computed } from 'vue'

const props = withDefaults(defineProps<{
  title?: string
  description?: string
  icon?: string
  siteName?: string
  theme?: string
  colorMode?: string
}>(), {
  colorMode: 'dark',
  icon: '/img/logo.png',
  theme: '#ff595e',
})

const HexRegex = /^#(?:[0-9a-f]{3}){1,2}$/i

const themeHex = computed(() => {
  if (HexRegex.test(props.theme))
    return props.theme
  if (HexRegex.test(`#${props.theme}`))
    return `#${props.theme}`
  if (props.theme.startsWith('rgb'))
    return rgbToHex(props.theme)
  return '#FFFFFF'
})

const themeRgb = computed(() => hexToRgb(themeHex.value))

const colorMode = computed(() => props.colorMode || 'dark')

const siteName = computed(() => props.siteName || useSiteConfig().name)
</script>

<template>
  <div
    class="w-full h-full flex justify-between relative p-[60px]"
    :class="colorMode === 'light' ? ['bg-white', 'text-gray-900'] : ['bg-gray-900', 'text-white']"
  >
    <div
      class="flex absolute top-0 right-[-100%]"
      :style="{
        width: '200%',
        height: '200%',
        backgroundImage: `radial-gradient(circle, rgba(${themeRgb}, 0.5) 0%, ${colorMode === 'dark' ? 'rgba(5, 5, 5,0.3)' : 'rgba(255, 255, 255, 0.7)'} 50%, ${colorMode === 'dark' ? 'rgba(5, 5, 5,0)' : 'rgba(255, 255, 255, 0)'} 70%)`,
      }"
    />
    <div class="h-full w-full justify-between relative">
      <div class="flex flex-row justify-between items-start">
        <div class="flex flex-col w-full max-w-[65%]">
          <h1 class="m-0 font-bold mb-[30px] text-[75px]">
            {{ title }}
          </h1>
          <p v-if="description" class="text-[35px]" :class="colorMode === 'light' ? 'text-gray-700' : 'text-gray-300'">
            {{ description }}
          </p>
        </div>
        <div v-if="icon" class="flex justify-end" style="width: 30%;">
          <img :src="icon" height="100" alt="Hack4Krak logo">
        </div>
      </div>
      <div class="flex flex-row justify-center items-center text-left w-full">
        <svg height="50" width="50" class="mr-3" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
          <path :fill="themeHex" d="M62.3,-53.9C74.4,-34.5,73.5,-9,67.1,13.8C60.6,36.5,48.7,56.5,30.7,66.1C12.7,75.7,-11.4,74.8,-31.6,65.2C-51.8,55.7,-67.9,37.4,-73.8,15.7C-79.6,-6,-75.1,-31.2,-61.1,-51C-47.1,-70.9,-23.6,-85.4,0.8,-86C25.1,-86.7,50.2,-73.4,62.3,-53.9Z" transform="translate(100 100)" />
        </svg>
        <p v-if="siteName" class="font-bold" style="font-size: 25px;">
          {{ siteName }}
        </p>
      </div>
    </div>
  </div>
</template>
