<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps({
  error: Object as () => NuxtError,
})

const errorCatImage = computed(() => {
  return `https://httpcat.us/static/${props.error?.statusCode}.png`
})

const errorMessage = computed(() => {
  switch (props.error?.statusCode) {
    case 404:
      return 'Uwaga rycerzu,\n ta strona zniknęła jak zamek w chmurach.\n Wróć na właściwą drogę!'
    case 500:
      return 'Rycerz napotkał przeszkodę\n na swojej drodze.\n Spróbuj ponownie za chwilę.'
    default:
      return props.error?.statusMessage
  }
})
</script>

<template>
  <NuxtLoadingIndicator color="#ffb900" :height="2" />
  <UApp class="font-roboto">
    <Navbar />
    <main class="min-h-[calc(100vh-var(--spacing)*19)] content-center items-center justify-center">
      <div class="flex content-center items-center justify-center">
        <div class="flex flex-col md:mr-32 w-3/4 md:max-w-2/5">
          <h1 class="text-balance text-8xl text-yellow-500 font-bold mb-3">
            {{ error?.statusCode }}
          </h1>
          <h2 class="whitespace-pre-line text-3xl text-black dark:text-white">
            {{ errorMessage }}
          </h2>
        </div>
        <div class="overflow-hidden border-1 border-white rounded-xl hidden md:block">
          <img class="w-125 h-90 object-cover scale-[110%]" :src="errorCatImage" alt="Error Cat">
        </div>
      </div>
    </main>
  </UApp>
</template>
