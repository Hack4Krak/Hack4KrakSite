<script setup lang="ts">
import type { NuxtError } from '#app'
import { setFavicon } from '~/utils/setFavicon'

const props = defineProps({
  error: Object as () => NuxtError,
})

useOgImage()

onMounted(() => {
  setFavicon()
})

const errorMessage = computed(() => {
  switch (props.error?.statusCode) {
    case 404:
      return 'Uwaga rycerzu,\n ta strona zniknęła jak zamek w chmurach.\n Wróć na właściwą drogę!'
    case 500:
      return 'Rycerz napotkał przeszkodę\n na swojej drodze.\n Spróbuj ponownie za chwilę.'
    default:
      return props.error?.message
  }
})
</script>

<template>
  <NuxtLayout name="default">
    <main class="content-center items-center justify-center min-h-[calc(100vh-var(--ui-header-height))]">
      <div class="flex content-center items-center justify-center flex-col-reverse md:flex-row">
        <div class="flex flex-col md:mr-10 max-w-3/4 md:max-w-2/5">
          <h1 class="text-balance text-8xl text-yellow-500 font-bold mb-3">
            {{ error?.statusCode }}
          </h1>
          <h2 class="whitespace-pre-line text-3xl text-white">
            {{ errorMessage }}
          </h2>
          <UModal title="Więcej informacji o błędzie:">
            <UButton label="Więcej informacji..." color="neutral" variant="subtle" class="w-fit mt-4 hover:bg-black/25 hover:text-yellow-700" />

            <template #body>
              <div class="border border-white border-b-white rounded-md m-2 overflow-auto bg-black/25">
                <div class="flex flex-col text-lg mb-6 ml-6">
                  <div class="text-xl font-bold text-yellow-600 my-5">
                    Kod:
                  </div>
                  <div class="ml-4 font-stretch-ultra-expanded font-light font-mono">
                    {{ error?.statusCode }}
                  </div>
                  <div class="text-xl font-bold text-yellow-600 my-5">
                    Wiadomość:
                  </div>
                  <div class="ml-4 font-stretch-ultra-expanded font-light font-mono">
                    {{ error?.message }}
                  </div>
                  <div class="text-xl font-bold text-yellow-600 my-5">
                    Dane:
                  </div>
                  <div class="ml-4 font-stretch-ultra-expanded font-light font-mono">
                    {{ error?.data }}
                  </div>
                </div>
              </div>
            </template>
          </UModal>
        </div>
        <div class="w-3/4 mb-10 md:w-150 md:mb-0 md:ml-10">
          <img class="w-full rendering-pixelated" src="assets/img/error_dragon.webp" alt="Dragon of (t)error">
        </div>
      </div>
    </main>
  </NuxtLayout>
</template>
