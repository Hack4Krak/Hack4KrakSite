<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps({
  error: Object as () => NuxtError,
})

const errorCatImage = computed(() => {
  return `https://http.cat/${props.error?.statusCode}`
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
    <main class="bg-gray-200 dark:bg-[#171717] min-h-[calc(100vh-var(--spacing)*19)] content-center items-center justify-center">
      <div class="flex content-center items-center justify-center">
        <div class="flex flex-col ml-12 mr-32 max-w-[600px]">
          <h1 class="text-balance text-8xl text-yellow-500 font-bold mb-10">
            {{ props.error?.statusCode }}
          </h1>
          <h2 class="whitespace-pre-line text-3xl text-black dark:text-white">
            {{ errorMessage }}
          </h2>
        </div>
        <img class="border-solid pl-10 border-white-8 rounded-xl object-cover scale-[120%]" :src="errorCatImage" alt="Error Cat">
      </div>
    </main>
  </UApp>
</template>
