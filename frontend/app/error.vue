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
  const error = props.error
  if (!error) {
    return 'Nieznany błąd'
  }
  const message = error.message?.toString().toLowerCase() || ''
  if (error.statusCode === 404) {
    if (message.includes('page not found')) {
      return 'Uwaga rycerzu,\n ta strona zniknęła jak zamek w chmurach.\n Wróć na właściwą drogę!'
    }
  }
  if (error.statusCode === 500) {
    return 'Rycerz napotkał przeszkodę\n Na swojej drodze.\n Spróbuj ponownie później.'
  }
  return error.statusMessage
})
</script>

<template>
  <NuxtLayout name="default">
    <div class="w-full mx-10 flex content-center items-center justify-center flex-col-reverse md:flex-row">
      <div class="flex flex-col md:mr-10 max-w-3/4 md:max-w-3/5 space-y-5">
        <h1 class="text-balance text-8xl text-primary font-bold">
          {{ error?.statusCode }}
        </h1>
        <h2 class="whitespace-pre-line text-2xl text-white">
          {{ errorMessage }}
        </h2>
        <LazyUModal title="Więcej informacji o błędzie:" hydrate-on-visible>
          <UButton label="Więcej informacji..." variant="outline" class="w-fit mt-4" />

          <template #body>
            <section class="flex flex-col text-lg space-y-5">
              <div
                v-for="(element, i) in [['Kod', error?.statusCode], ['Wiadomość', error?.statusMessage], ['Dane', error?.data]]"
                :key="i"
              >
                <h2 class="text-xl font-bold text-primary">
                  {{ element[0] }}:
                </h2>
                <pre class="font-light font-mono">{{ element[1] }}</pre>
              </div>
            </section>
          </template>
        </LazyUModal>
      </div>

      <div class="w-3/4 mb-10 md:w-150 md:mb-0 md:ml-10">
        <img class="w-full rendering-pixelated" src="assets/img/error_dragon.webp" alt="Dragon of (t)error">
      </div>
    </div>
  </NuxtLayout>
</template>
