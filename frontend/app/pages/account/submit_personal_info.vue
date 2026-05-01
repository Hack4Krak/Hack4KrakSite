<script setup lang="ts">
import type { AuthRequestBody } from '#open-fetch'
import type { SchemaCtfExperience, SchemaSchoolGrade } from '#open-fetch-schemas/api'
import { FetchError } from 'ofetch'

type FormState = NonNullable<AuthRequestBody<'submit_personal_information'>>

const STEP_TITLES = ['Powitanie', 'O Tobie', 'Jak nas znalazłeś?', 'Na koniec']

const stepIndex = ref(0)
const direction = ref<'forward' | 'backward'>('forward')
const submitting = ref(false)

const form = reactive<FormState>({
  first_name: '',
  location: '',
  ctf_experience: 'Never' as SchemaCtfExperience,
  school_grade: 'NotStudying' as SchemaSchoolGrade,
  referral_sources: [],
  marketing_consent: false,
  collab_interest: false,
})

const { data: existing } = await useAuth('/account/get_personal_information', {
  onResponseError: undefined,
})

if (existing.value) {
  Object.assign(form, {
    first_name: existing.value.first_name ?? '',
    location: existing.value.location ?? '',
    ctf_experience: existing.value.ctf_experience ?? 'Never',
    school_grade: existing.value.school_grade ?? 'NotStudying',
    referral_sources: (existing.value.referral_sources as string[] | null) ?? [],
    marketing_consent: existing.value.marketing_consent ?? false,
    collab_interest: existing.value.collab_interest ?? false,
  })
}

const stepValidations = computed(() => [
  true,
  form.first_name.trim().length > 0 && form.location.trim().length > 0,
  form.referral_sources.length > 0,
  true,
])

const canAdvance = computed(() => stepValidations.value[stepIndex.value])
const progress = computed(() => ((stepIndex.value + 1) / STEP_TITLES.length) * 100)

function next() {
  if (!canAdvance.value)
    return
  if (stepIndex.value < STEP_TITLES.length - 1) {
    direction.value = 'forward'
    stepIndex.value++
  }
}

function prev() {
  if (stepIndex.value > 0) {
    direction.value = 'backward'
    stepIndex.value--
  }
}

async function onSubmit() {
  submitting.value = true
  try {
    await useNuxtApp().$auth('/account/submit_personal_information', {
      method: 'POST',
      credentials: 'include',
      body: { ...form },
    })

    useToast().add({
      title: 'Gotowe!',
      description: 'Dzięki za poznanie się. Powodzenia w zawodach!',
      color: 'success',
    })

    await refreshNuxtData()
    await navigateTo('/panel')
  } catch (error) {
    if (!(error instanceof FetchError)) {
      throw error
    }
  } finally {
    submitting.value = false
  }
}

useHead({ title: 'Witaj w Hack4Krak' })
</script>

<template>
  <div class="md:w-[40rem] sm:w-96 w-72">
    <div class="mb-6 space-y-2">
      <div class="flex items-center justify-between text-sm text-muted">
        <span class="font-pixelify uppercase tracking-wider">
          Krok {{ stepIndex + 1 }} z {{ STEP_TITLES.length }}
        </span>
        <span v-if="stepIndex > 0" class="opacity-70">
          {{ STEP_TITLES[stepIndex] }}
        </span>
      </div>
      <div class="relative h-1.5 bg-elevated overflow-hidden">
        <div
          class="absolute inset-y-0 left-0 bg-primary transition-[width] duration-500 ease-out"
          :style="{ width: `${progress}%` }"
        />
      </div>
    </div>

    <div class="relative">
      <Transition :name="direction === 'forward' ? 'step-forward' : 'step-backward'" mode="out-in">
        <OnboardingStageWelcome v-if="stepIndex === 0" key="welcome" />
        <OnboardingStageAbout
          v-else-if="stepIndex === 1"
          key="about"
          v-model:first-name="form.first_name"
          v-model:location="form.location"
          v-model:ctf-experience="form.ctf_experience"
          v-model:school-grade="form.school_grade"
          @submit="next"
        />
        <OnboardingStageSource
          v-else-if="stepIndex === 2"
          key="source"
          v-model:sources="form.referral_sources"
        />
        <OnboardingStageConsents
          v-else
          key="consents"
          v-model:marketing-consent="form.marketing_consent"
          v-model:collab-interest="form.collab_interest"
        />
      </Transition>
    </div>

    <div class="flex items-center gap-3 mt-8 pt-4 border-t border-default">
      <UButton
        v-if="stepIndex > 0"
        color="neutral"
        variant="ghost"
        icon="lucide:arrow-left"
        :disabled="submitting"
        @click="prev"
      >
        Wstecz
      </UButton>

      <div class="flex-1" />

      <UButton
        v-if="stepIndex < STEP_TITLES.length - 1"
        color="primary"
        size="lg"
        trailing-icon="lucide:arrow-right"
        :disabled="!canAdvance"
        @click="next"
      >
        {{ stepIndex === 0 ? 'Zaczynajmy' : 'Dalej' }}
      </UButton>
      <UButton
        v-else
        color="primary"
        size="lg"
        trailing-icon="lucide:check"
        :loading="submitting"
        @click="onSubmit"
      >
        Zapisz i wejdź
      </UButton>
    </div>
  </div>
</template>

<style scoped>
.step-forward-enter-active,
.step-forward-leave-active,
.step-backward-enter-active,
.step-backward-leave-active {
  transition:
    opacity 120ms ease,
    transform 160ms cubic-bezier(0.22, 1, 0.36, 1);
}

.step-forward-enter-from {
  opacity: 0;
  transform: translateX(0.75rem);
}
.step-forward-leave-to {
  opacity: 0;
  transform: translateX(-0.75rem);
}
.step-backward-enter-from {
  opacity: 0;
  transform: translateX(-0.75rem);
}
.step-backward-leave-to {
  opacity: 0;
  transform: translateX(0.75rem);
}
</style>
