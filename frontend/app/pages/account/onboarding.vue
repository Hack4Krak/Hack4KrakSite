<script setup lang="ts">
import type { AuthRequestBody } from '#open-fetch'
import type { SchemaCtfExperience, SchemaSchoolGrade } from '#open-fetch-schemas/api'
import { FetchError } from 'ofetch'

definePageMeta({
  layout: 'centered',
})

type FormState = NonNullable<AuthRequestBody<'submit_onboarding'>>

const STEP_TITLES = ['Powitanie', 'Pytania profilujące', 'Jak nas znalazłeś?', 'Na koniec']

const stepIndex = ref(0)
const direction = ref<'forward' | 'backward'>('forward')
const submitting = ref(false)

const form = reactive<FormState>({
  organization: '',
  location: '',
  ctf_experience: 'Never' as SchemaCtfExperience,
  school_grade: 'NotStudying' as SchemaSchoolGrade,
  referral_sources: [],
  marketing_consent: false,
  collab_interest: false,
})

const stepValidations = computed(() => [
  true,
  form.organization.trim().length > 0 && form.location.trim().length > 0,
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
    await useNuxtApp().$auth('/account/onboarding', {
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
  <div class="w-full sm:w-96 md:w-[40rem]">
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
          v-model:organization="form.organization"
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
