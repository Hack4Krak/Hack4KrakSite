<script setup lang="ts">
import type { components } from '#open-fetch-schemas/api'
import { FetchError } from 'ofetch'

definePageMeta({
  middleware: 'admin',
})

type VerifiedUserInfo
  = components['schemas']['VerifiedUserInfo']

const { $auth } = useNuxtApp()

const { data: availableTags } = await useApi('/event/participant-tags')

const selectionState = ref<{
  mode: 'identify' | 'apply-tag'
  selectedTagId: string
  verificationId: string
  userInfo: VerifiedUserInfo | null
}>({
  mode: 'identify',
  selectedTagId: '',
  verificationId: '',
  userInfo: null,
})

const showScanner = ref(false)
const isLoading = ref(false)

function resetState() {
  selectionState.value = {
    mode: selectionState.value.mode,
    selectedTagId: selectionState.value.selectedTagId,
    verificationId: '',
    userInfo: null,
  }
}

function onCodeScanned(code: string) {
  selectionState.value.verificationId = code
  showScanner.value = false
  handleSubmit()
}

async function handleSubmit() {
  if (!selectionState.value.verificationId) {
    useToast().add({
      title: 'Błąd',
      description: 'Wprowadź kod weryfikacyjny UUID',
      color: 'error',
    })
    return
  }

  isLoading.value = true

  try {
    const response = await $auth('/admin/verification/identify', {
      method: 'POST',
      body: {
        verification_id: selectionState.value.verificationId,
      },
      ignoreResponseError: false,
    })

    selectionState.value.userInfo = response

    useToast().add({
      title: 'Użytkownik zidentyfikowany',
      description: `${response.username}`,
      color: 'success',
    })
  } catch (error: unknown) {
    const errorMessage = (error as { data?: { message?: string } })?.data?.message || 'Nieznany błąd'
    useToast().add({
      title: 'Błąd',
      description: errorMessage,
      color: 'error',
    })
    resetState()
  } finally {
    isLoading.value = false
  }
}

async function confirmTagApplication() {
  if (!selectionState.value.userInfo || !selectionState.value.selectedTagId) {
    return
  }

  isLoading.value = true

  try {
    await $auth('/admin/verification/apply-tag', {
      method: 'POST',
      body: {
        verification_id: selectionState.value.verificationId,
        tag_id: selectionState.value.selectedTagId,
      },
      ignoreResponseError: false,
    })

    useToast().add({
      title: 'Tag przypisany',
      description: `Tag "${selectionState.value.selectedTagId}" został przypisany do użytkownika ${selectionState.value.userInfo?.username}`,
      color: 'success',
    })

    resetState()
  } catch (error) {
    console.error(error)
    if (!(error instanceof FetchError)) {
      throw error
    }
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="md:w-2xl md:min-w-2xl w-screen mx-auto p-6">
    <h1 class="text-2xl font-bold mb-6">
      Weryfikacja uczestników
    </h1>

    <div class="mb-6">
      <label class="block text-sm font-medium mb-2">Tryb</label>
      <div class="flex gap-4">
        <UButton
          :variant="selectionState.mode === 'identify' ? 'solid' : 'outline'"
          color="primary"
          @click="selectionState.mode = 'identify'; resetState()"
        >
          Identyfikacja
        </UButton>
        <UButton
          :variant="selectionState.mode === 'apply-tag' ? 'solid' : 'outline'"
          color="primary"
          @click="selectionState.mode = 'apply-tag'; resetState()"
        >
          Przypisanie tagu
        </UButton>
      </div>
    </div>

    <div v-if="selectionState.mode === 'apply-tag'" class="mb-6">
      <label class="block text-sm font-medium mb-2">Wybierz tag</label>
      <USelect
        v-model="selectionState.selectedTagId"
        :items="availableTags?.map(t => ({ label: t.name, value: t.id, description: t.description })) || []"
        placeholder="Wybierz tag..."
        class="w-full"
        value-key="value"
      />
    </div>

    <div class="mb-6">
      <label class="block text-sm font-medium mb-2">Kod weryfikacyjny UUID</label>
      <div class="flex gap-2">
        <UInput
          v-model="selectionState.verificationId"
          placeholder="Wpisz lub zeskanuj UUID..."
          class="flex-1"
          @keyup.enter="handleSubmit"
        />
        <UButton
          icon="mdi:qrcode-scan"
          color="neutral"
          variant="outline"
          @click="showScanner = true"
        >
          Skanuj
        </UButton>
      </div>
    </div>

    <div class="mb-6">
      <UButton
        :loading="isLoading"
        color="primary"
        class="w-full"
        @click="handleSubmit"
      >
        {{ selectionState.mode === 'identify' ? 'Zidentyfikuj' : 'Znajdź użytkownika' }}
      </UButton>
    </div>

    <div v-if="selectionState.userInfo" class="mb-6 p-4 rounded-lg bg-elevated border border-accented">
      <h2 class="text-lg font-semibold mb-3">
        Informacje o użytkowniku
      </h2>
      <div class="space-y-2">
        <p><span class="font-medium">Nazwa użytkownika:</span> {{ selectionState.userInfo.username }}</p>
        <p><span class="font-medium">Email:</span> {{ selectionState.userInfo.email }}</p>
        <p><span class="font-medium">Drużyna:</span> {{ selectionState.userInfo.team_name || 'Brak' }}</p>
      </div>

      <div v-if="selectionState.selectedTagId" class="mt-4 pt-4 border-t border-accented">
        <p class="mb-3">
          Czy chcesz przypisać tag <strong>{{ selectionState.selectedTagId }}</strong> do tego użytkownika?
        </p>
        <div class="flex gap-2">
          <UButton
            :loading="isLoading"
            color="primary"
            @click="confirmTagApplication"
          >
            Potwierdź
          </UButton>
          <UButton
            color="neutral"
            variant="outline"
            @click="selectionState.selectedTagId = ''"
          >
            Anuluj
          </UButton>
        </div>
      </div>
    </div>

    <PanelModalQRCodeScan
      v-model="showScanner"
      @code-scanned="onCodeScanned"
    />
  </div>
</template>
