<script setup lang="ts">
import type { SchemaIdentifiedUserInfo } from '#open-fetch-schemas/api'
import type { DetectedBarcode } from 'nuxt-qrcode'
import { FetchError } from 'ofetch'

definePageMeta({
  middleware: 'admin',
})

const { $auth } = useNuxtApp()

const { data: availableTags } = await useApi('/event/participant-tags')

const mode = ref<'identify' | 'apply-tag'>('identify')
const selectedTagId = ref('')
const userInfo = ref<SchemaIdentifiedUserInfo | null>(null)
const currentId = ref('')
const isLoading = ref(false)

const userAlreadyHasSelectedTag = computed(() =>
  userInfo.value?.tags.some(t => t.id === selectedTagId.value) ?? false,
)

const selectedTagLabel = computed(() =>
  availableTags.value?.find(t => t.id === selectedTagId.value)?.name ?? selectedTagId.value,
)

const foodPreferenceLabel = computed(() => {
  switch (userInfo.value?.food_preference) {
    case 'Standard':
      return 'Standardowe'
    case 'Vegetarian':
      return 'Wegetariańskie'
    default:
      return userInfo.value?.food_preference ?? '—'
  }
})

const participantAge = computed(() => {
  const birthYear = Number(userInfo.value?.birth_year)
  if (!birthYear || Number.isNaN(birthYear))
    return '—'

  return String(new Date().getFullYear() - birthYear)
})

function dismiss() {
  userInfo.value = null
  currentId.value = ''
}

function switchMode(newMode: 'identify' | 'apply-tag') {
  mode.value = newMode
  dismiss()
}

async function onDetect(detectedCodes: DetectedBarcode[]) {
  if (!detectedCodes.length || isLoading.value || userInfo.value)
    return
  if (mode.value === 'apply-tag' && !selectedTagId.value)
    return

  const code = detectedCodes[0]!.rawValue
  currentId.value = code

  identify()
}

async function identify() {
  isLoading.value = true
  try {
    userInfo.value = await $auth('/admin/identification/identify/{id}', {
      path: { id: currentId.value },
      ignoreResponseError: false,
    })
  } catch (error: unknown) {
    const msg = (error as { data?: { message?: string } })?.data?.message || 'Nieznany błąd'
    useToast().add({ title: 'Błąd identyfikacji', description: msg, color: 'error' })
  } finally {
    isLoading.value = false
  }
}

async function applyTag() {
  if (!userInfo.value || !selectedTagId.value)
    return

  isLoading.value = true
  try {
    await $auth('/admin/identification/apply-tag/{id}', {
      method: 'POST',
      body: { tag_id: selectedTagId.value },
      path: { id: currentId.value },
      ignoreResponseError: false,
    })
    useToast().add({
      title: 'Tag przypisany',
      description: `${userInfo.value.username} ← ${selectedTagLabel.value}`,
      color: 'success',
    })
  } catch (error) {
    if (!(error instanceof FetchError))
      throw error
    const msg = error.data?.message || 'Nieznany błąd'
    useToast().add({ title: 'Błąd przypisania tagu', description: msg, color: 'error' })
  } finally {
    isLoading.value = false
  }

  dismiss()
}

function onCameraError(error: Error) {
  useToast().add({ title: 'Błąd kamery', description: error.message, color: 'error' })
}
</script>

<template>
  <div
    class="flex flex-col w-full md:max-w-lg md:mx-auto"
    style="height: calc(100dvh - var(--ui-header-height, 4rem))"
  >
    <!-- Controls -->
    <div class="flex-shrink-0 p-3 flex flex-col gap-2 border-b border-accented">
      <div class="flex gap-2">
        <UButton
          size="sm"
          :variant="mode === 'identify' ? 'solid' : 'outline'"
          color="primary"
          class="flex-1"
          @click="switchMode('identify')"
        >
          Identyfikacja
        </UButton>
        <UButton
          size="sm"
          :variant="mode === 'apply-tag' ? 'solid' : 'outline'"
          color="primary"
          class="flex-1"
          @click="switchMode('apply-tag')"
        >
          Przypisanie tagu
        </UButton>
      </div>
      <USelect
        v-if="mode === 'apply-tag'"
        v-model="selectedTagId"
        :items="availableTags?.map(t => ({ label: t.name, value: t.id })) || []"
        placeholder="Wybierz tag do przypisania..."
        size="sm"
        value-key="value"
        class="w-full"
      />
      <UInput v-model="currentId" placeholder="Ręczne wpisywanie kodu" size="sm" class="w-full" />
      <UButton
        size="sm" class="w-full" :loading="isLoading" @click="identify"
      >
        Zidentyfikuj
      </UButton>
    </div>

    <div class="flex-1 relative min-h-0 bg-background">
      <div
        v-if="!userInfo"
        class="absolute inset-0 bg-black"
      >
        <QrcodeStream
          :paused="isLoading"
          class="absolute inset-0 w-full h-full"
          @detect="onDetect"
          @error="onCameraError"
        />

        <div
          v-if="isLoading"
          class="absolute inset-0 flex items-center justify-center bg-black/60"
        >
          <UIcon
            name="mdi:loading"
            class="animate-spin text-white size-12"
          />
        </div>

        <div
          v-if="mode === 'apply-tag' && !selectedTagId"
          class="absolute inset-0 bg-black/50 flex items-center justify-center pointer-events-none"
        >
          <p class="text-white/80 text-base text-center px-6">
            Wybierz tag, aby aktywować aparat
          </p>
        </div>
      </div>

      <div
        v-else
        class="absolute inset-0 overflow-y-auto p-3 sm:p-5"
      >
        <div class="space-y-4">
          <div class="bg-surface-muted border-2 border-surface-muted">
            <div class="bg-default p-5">
              <p class="font-pixelify text-xs uppercase tracking-[0.25em] text-success">
                zeskanowano
              </p>
              <h1 class="mt-2 break-words text-3xl font-bold tracking-tight">
                {{ userInfo.full_name }}
              </h1>
              <p class="mt-2 break-words text-sm text-muted">
                {{ userInfo.username }}
              </p>
              <p class="mt-1 break-words text-sm text-muted">
                {{ userInfo.email }}
              </p>
              <p class="mt-3 inline-flex items-center border px-2 py-1 text-sm font-medium" :class="userInfo.is_underage ? 'border-warning text-warning bg-warning/10' : 'border-success text-success bg-success/10'">
                {{ userInfo.is_underage ? 'Niepełnoletni' : 'Pełnoletni' }}
              </p>
            </div>

            <div class="space-y-px">
              <div class="bg-default p-5 flex items-center justify-between gap-4">
                <div>
                  <p class="font-medium">
                    Dane uczestnika
                  </p>
                  <dl class="mt-2 grid grid-cols-1 gap-2 text-sm sm:grid-cols-2">
                    <div>
                      <dt class="text-muted">
                        Szkoła
                      </dt>
                      <dd class="break-words">
                        {{ userInfo.school || '—' }}
                      </dd>
                    </div>
                    <div>
                      <dt class="text-muted">
                        Wiek / rok urodzenia
                      </dt>
                      <dd>
                        {{ participantAge }} / {{ userInfo.birth_year || '—' }}
                      </dd>
                    </div>
                  </dl>
                </div>
                <UIcon
                  name="mdi:account-details"
                  class="size-5 shrink-0 text-muted"
                />
              </div>

              <div class="bg-default p-5 flex items-center justify-between gap-4">
                <div>
                  <p class="font-medium">
                    Preferencje żywieniowe
                  </p>
                  <p class="text-sm text-muted mt-1">
                    {{ foodPreferenceLabel }}
                  </p>
                  <p v-if="userInfo.food_allergies" class="text-sm text-muted mt-1 break-words">
                    Alergie: {{ userInfo.food_allergies }}
                  </p>
                </div>
                <UIcon
                  name="mdi:food-apple"
                  class="size-5 shrink-0 text-muted"
                />
              </div>

              <div class="bg-default p-5 flex items-center justify-between gap-4">
                <div>
                  <p class="font-medium">
                    Drużyna
                  </p>
                  <p class="text-sm text-muted mt-1">
                    {{ userInfo.team_name || 'Brak drużyny' }}
                  </p>
                </div>
                <UIcon
                  name="pixelarticons:users"
                  class="size-5 shrink-0 text-muted"
                />
              </div>

              <div class="bg-default p-5 flex items-center justify-between gap-4">
                <div>
                  <p class="font-medium">
                    Kod identyfikacyjny
                  </p>
                  <p class="text-sm text-muted mt-1 break-all">
                    {{ currentId }}
                  </p>
                </div>
                <UIcon
                  name="mdi:qrcode-scan"
                  class="size-5 shrink-0 text-muted"
                />
              </div>
            </div>
          </div>

          <PanelTileParticipantTags
            :all-tags="availableTags ?? []"
            :applied-tags="userInfo.tags"
          />

          <div
            v-if="mode === 'apply-tag' && selectedTagId"
            class="border p-4"
            :class="userAlreadyHasSelectedTag ? 'border-warning bg-warning/10 text-warning' : 'border-primary bg-primary/10 text-default'"
          >
            <p
              v-if="userAlreadyHasSelectedTag"
              class="flex items-start gap-2 text-sm"
            >
              <UIcon
                name="mdi:alert-circle"
                class="mt-0.5 size-5 shrink-0"
              />
              <span>Tag <strong>{{ selectedTagLabel }}</strong> jest już przypisany.</span>
            </p>
            <p
              v-else
              class="flex items-start gap-2 text-sm"
            >
              <UIcon
                name="mdi:tag-plus"
                class="mt-0.5 size-5 shrink-0 text-primary"
              />
              <span>Zostanie przypisany: <strong>{{ selectedTagLabel }}</strong></span>
            </p>
          </div>

          <div
            class="flex gap-3"
            :class="mode === 'apply-tag' ? 'flex-col' : ''"
          >
            <template v-if="mode === 'apply-tag'">
              <UButton
                icon="mdi:check"
                color="success"
                size="xl"
                class="w-full justify-center"
                :loading="isLoading"
                :disabled="isLoading"
                @click="applyTag"
              >
                Przypisz tag
              </UButton>
              <UButton
                icon="mdi:close"
                color="neutral"
                variant="outline"
                size="lg"
                class="w-full justify-center"
                :disabled="isLoading"
                @click="dismiss"
              >
                Anuluj
              </UButton>
            </template>

            <UButton
              v-else
              icon="mdi:qrcode-scan"
              color="primary"
              size="xl"
              class="w-full justify-center"
              @click="dismiss"
            >
              Skanuj dalej
            </UButton>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
