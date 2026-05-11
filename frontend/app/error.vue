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

const eventStartDate = ref<Date | undefined>(undefined)
const errorData = computed<Record<string, any>>(() => {
  return props.error?.data || {}
})
const errorTitle = ref({
  title: 'Błąd',
  message: 'Nieznany błąd',
  titleClass: 'text-8xl',
})

watchEffect(async () => {
  const error = props.error
  if (!error)
    return

  console.error(error)

  const customErrorId = errorData.value?.error
  if (customErrorId === 'AccessBeforeEventStart') {
    const [eventStart, _] = await useEventStartAndEnd()
    eventStartDate.value = eventStart
    errorTitle.value = {
      title: 'Skąd ten pośpiech?',
      message: 'CTF jeszcze się nie rozpoczął!\n Czas do początku wydarzenia:',
      titleClass: 'text-5xl',
    }
    return
  }

  const status = error.status
  errorTitle.value.title = String(status ?? 'Błąd')

  if (error.status === 404) {
    errorTitle.value.message
      = 'Uwaga rycerzu,\n ta strona zniknęła jak zamek w chmurach.\n Wróć na właściwą drogę!'
  } else if (error.status === 500) {
    errorTitle.value.message = 'Rycerz napotkał przeszkodę\n Na swojej drodze.\n Spróbuj ponownie później.'
  } else if (errorData.value.message) {
    errorTitle.value.message = errorData.value.message
  } else if (error.message) {
    errorTitle.value.message = error.message
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

const rawErrorJson = computed(() => JSON.stringify(props.error, null, 2))

async function copyRaw() {
  await navigator.clipboard.writeText(rawErrorJson.value)
  useToast().add({ title: 'Skopiowano', icon: 'i-lucide-check', color: 'success' })
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

        <LazyUModal v-else hydrate-on-visible :ui="{ content: 'max-w-2xl' }">
          <UButton label="Więcej informacji..." variant="outline" class="w-fit mt-4" />
          <template #title>
            <span class="flex items-center gap-2 font-pixelify text-lg tracking-wide">
              <UIcon name="pixelarticons:bug" class="size-5 text-primary" />
              Szczegóły błędu
            </span>
          </template>
          <template #body>
            <div class="relative">
              <pre class="font-mono text-xs bg-elevated p-3 overflow-auto max-h-96">{{ rawErrorJson }}</pre>
              <UButton icon="i-lucide-copy" variant="ghost" size="xs" color="neutral" class="absolute top-2 right-2" @click="copyRaw" />
            </div>
          </template>
        </LazyUModal>
      </div>

      <div class="w-3/4 mb-10 md:w-150 md:mb-0 md:ml-10">
        <img class="w-full rendering-pixelated" src="assets/img/error_dragon.webp" alt="Dragon of (t)error">
      </div>
    </div>
  </NuxtLayout>
</template>
