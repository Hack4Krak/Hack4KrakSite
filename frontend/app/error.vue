<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps({
  error: Object as () => NuxtError,
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
    <main class="bg-[#18181b] min-h-[calc(100vh-var(--spacing)*19)] content-center items-center justify-center">
      <div class="flex content-center items-center justify-center">
        <div class="flex flex-col md:mr-10 md:max-w-2/5">
          <h1 class="text-balance text-8xl text-yellow-500 font-bold mb-3">
            {{ error?.statusCode }}
          </h1>
          <h2 class="whitespace-pre-line text-3xl text-white">
            {{ errorMessage }}
          </h2>
        </div>
        <div class="hidden lg:block md:ml-10">
          <img class="w-150 rendering-pixelated" src="assets/img/error_dragon2.webp" alt="Error Dragon">
        </div>
      </div>
    </main>
  </UApp>
</template>
