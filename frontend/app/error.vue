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

  const customErrorId = errorData.value.response?.error
  if (customErrorId === 'AccessBeforeEventStart') {
    eventStartDate.value = new Date(errorData.value?.response.details.event_start_date)
    errorTitle.value = {
      title: 'Skąd ten pośpiech?',
      message: 'CTF jeszcze się nie rozpoczął!\n Czas do początku wydarzenia:',
      titleClass: 'text-5xl',
    }
    return
  }

  const message = error.message?.toString().toLowerCase() || ''
  errorTitle.value.title = error.statusCode.toString()

  if (error.statusCode === 404) {
    if (message.includes('page not found')) {
      errorTitle.value.message
        = 'Uwaga rycerzu,\n ta strona zniknęła jak zamek w chmurach.\n Wróć na właściwą drogę!'
    }
  } else if (error.statusCode === 500) {
    errorTitle.value.message = 'Rycerz napotkał przeszkodę\n Na swojej drodze.\n Spróbuj ponownie później.'
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
    <div class="w-full mx-10 flex content-center items-center justify-center flex-col-reverse md:flex-row">
      <div class="flex flex-col md:mr-10 max-w-3/4 md:max-w-3/5 space-y-5">
        <h1 class="text-balance text-5xl text-primary font-bold" :class="errorTitle.titleClass">
          {{ errorTitle.title }}
        </h1>
        <h2 class="whitespace-pre-line text-2xl text-white">
          {{ errorTitle.message }}
        </h2>

        <LazyTimer v-if="eventStartDate" class="mt-10" :target="eventStartDate" @complete="finishTimer()" />

        <LazyUModal v-else title="Więcej informacji o błędzie:" hydrate-on-visible>
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
