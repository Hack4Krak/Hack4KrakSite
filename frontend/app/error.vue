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

const eventStartDate = ref<Date | null>(null)
const errorData = computed<Record<string, any>>(() => {
  return props.error?.data || {}
})
const errorTitle = ref({
  title: 'Błąd',
  message: 'Nieznany błąd',
  titleClass: 'text-8xl',
})

watchEffect(() => {
  const error = props.error
  if (!error)
    return

  const customErrorId = errorData.value?.error
  if (customErrorId === 'AccessBeforeStage') {
    eventStartDate.value = new Date(errorData.value?.details?.stage_start_date)
    errorTitle.value = {
      title: 'Skąd ten pośpiech?',
      message: 'CTF jeszcze się nie rozpoczął!\n Czas do początku wydarzenia:',
      titleClass: 'text-5xl',
    }
    return
  }

  const nuxtErrorMessage = error.message?.toString().toLowerCase() || ''
  const status = error.status
  errorTitle.value.title = String(status ?? 'Błąd')

  if (error.status === 404) {
    if (nuxtErrorMessage.includes('page not found')) {
      errorTitle.value.message
        = 'Uwaga rycerzu,\n ta strona zniknęła jak zamek w chmurach.\n Wróć na właściwą drogę!'
    }
  } else if (error.status === 403) {
    errorTitle.value.message = 'Nie masz uprawnień,\n aby zobaczyć tę stronę.'
  } else if (error.status === 500) {
    errorTitle.value.message = 'Rycerz napotkał przeszkodę\n Na swojej drodze.\n Spróbuj ponownie później.'
  } else if (errorData.value.message) {
    errorTitle.value.message = errorData.value.message
  }
})

async function finishTimer() {
  await refreshNuxtData()
  await clearError()
  useToast().add({
    title: 'Czas, start!',
    description: 'Miłego rozwiązywania zadań :3',
    color: 'success',
  })
}
</script>

<template>
  <NuxtLayout name="default">
    <div class="w-full flex-1 flex content-center items-center justify-center flex-col-reverse md:flex-row">
      <div class="flex flex-col md:mr-10 max-w-3/4 md:max-w-3/5 space-y-5">
        <h1 class="text-balance text-5xl text-primary font-bold" :class="errorTitle.titleClass">
          {{ errorTitle.title }}
        </h1>
        <h2 class="whitespace-pre-line text-2xl text-white">
          {{ errorTitle.message }}
        </h2>

        <LazyTimer v-if="eventStartDate" class="mt-10" :target="eventStartDate" @complete="finishTimer()" />

        <div v-else class="flex flex-wrap gap-3 mt-4">
          <UButton label="Wróć na stronę główną" to="/" />
          <LazyUModal title="Więcej informacji o błędzie:" hydrate-on-visible>
            <UButton label="Więcej informacji..." variant="outline" />
            <template #body>
              <section class="flex flex-col text-lg space-y-5">
                <div
                  v-for="(element, i) in [['Kod', error?.status], ['Wiadomość', error?.statusText], ['Dane', error?.data]]"
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
      </div>

      <div class="w-3/4 mb-10 md:w-150 md:mb-0 md:ml-10">
        <img class="w-full rendering-pixelated" src="assets/img/error_dragon.webp" alt="Dragon of (t)error">
      </div>
    </div>
  </NuxtLayout>
</template>
