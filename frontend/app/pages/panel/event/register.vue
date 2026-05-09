<script setup lang="ts">
definePageMeta({
  layout: 'panel',
  middleware: ['only-unregistered-participants'],
})

useSeoMeta({
  title: 'Rejestracja na Hack4Krak 2026',
  description: 'Zapisz drużynę na nadchodzącą edycję Hack4Krak CTF.',
})

const {
  draft,
  isRegistered,
  submitRegistration,
} = useEventRegistration()

const STEPS = [
  { id: 'agreements', label: 'Zgody na udział', short: 'Zgody' },
  { id: 'personal', label: 'Twoje dane', short: 'O Tobie' },
  { id: 'team', label: 'Drużyna', short: 'Drużyna' },
  { id: 'confirmation', label: 'Potwierdzenie', short: 'Na koniec' },
] as const

type StepId = (typeof STEPS)[number]['id']

const stepIndex = ref(0)
const direction = ref<'forward' | 'backward'>('forward')
const validating = ref(false)
const submitting = ref(false)

const currentStep = computed<StepId>(() => STEPS[stepIndex.value]!.id)

interface RegistrationStepRef {
  validate: () => Promise<void>
}

const agreementsStep = useTemplateRef<RegistrationStepRef>('agreementsStep')
const personalStep = useTemplateRef<RegistrationStepRef>('personalStep')
const confirmationStep = useTemplateRef<RegistrationStepRef>('confirmationStep')

async function validateCurrentStep() {
  const stepRef = currentStep.value === 'agreements'
    ? agreementsStep.value
    : currentStep.value === 'personal'
      ? personalStep.value
      : currentStep.value === 'confirmation'
        ? confirmationStep.value
        : undefined

  if (!stepRef)
    return true

  validating.value = true
  try {
    await stepRef.validate()
    return true
  } catch {
    return false
  } finally {
    validating.value = false
  }
}

async function next() {
  if (!await validateCurrentStep())
    return
  if (stepIndex.value < STEPS.length - 1) {
    direction.value = 'forward'
    stepIndex.value++
  }
}

async function prev() {
  if (stepIndex.value === 0) {
    await navigateTo('/account/events')
    return
  }
  direction.value = 'backward'
  stepIndex.value--
}

async function goToStep(index: number) {
  if (index === stepIndex.value)
    return
  if (index > stepIndex.value + 1)
    return
  if (index > stepIndex.value && !await validateCurrentStep())
    return
  direction.value = index > stepIndex.value ? 'forward' : 'backward'
  stepIndex.value = index
}

async function onSubmit() {
  if (!await validateCurrentStep())
    return

  submitting.value = true
  try {
    await submitRegistration()

    useToast().add({
      title: 'Zgłoszenie zapisane',
      description: 'Twoje dane zostały przesłane. Do zobaczenia na wydarzeniu!',
      color: 'success',
    })

    await navigateTo('/panel/event')
  } catch {
    // errors shown as toasts by the $auth plugin
  } finally {
    submitting.value = false
  }
}

if (isRegistered.value) {
  await navigateTo('/panel/event')
}
</script>

<template>
  <div class="m-4 lg:m-8 border-2 border-surface-muted bg-default min-h-[calc(100vh-12rem)] flex flex-col">
    <header class="px-6 lg:px-10 pt-6 lg:pt-7 pb-5 border-b-2 border-surface-muted space-y-4">
      <div class="flex items-baseline justify-between gap-4">
        <p class="font-pixelify uppercase tracking-wider text-sm">
          Krok <span class="text-primary font-bold">{{ stepIndex + 1 }}</span> z {{ STEPS.length }}
          <span class="text-muted ml-2 normal-case tracking-normal font-sans text-xs">
            · {{ STEPS[stepIndex]!.label }}
          </span>
        </p>
        <p class="text-xs text-muted hidden sm:block">
          Hack4Krak 2026
        </p>
      </div>

      <UProgress
        :model-value="stepIndex + 1"
        :max="STEPS.length"
        size="sm"
        :ui="{ root: 'rounded-none', base: 'rounded-none bg-surface-muted', indicator: 'rounded-none transition-[width] duration-500 ease-out' }"
      />

      <ol class="grid grid-cols-4 gap-4">
        <li
          v-for="(step, index) in STEPS"
          :key="step.id"
        >
          <button
            type="button"
            :disabled="index > stepIndex + 1 || validating || submitting"
            class="w-full flex items-center gap-2 text-xs uppercase tracking-wider font-bold cursor-pointer disabled:cursor-not-allowed transition-colors"
            :class="index === stepIndex
              ? 'text-primary'
              : index < stepIndex
                ? 'text-default hover:text-primary'
                : 'text-muted/50'"
            @click="goToStep(index)"
          >
            <span
              class="flex size-5 shrink-0 items-center justify-center border-2 text-xs transition-colors"
              :class="index < stepIndex
                ? 'bg-primary border-primary text-default'
                : index === stepIndex
                  ? 'border-primary text-primary'
                  : 'border-surface-muted text-muted/50'"
            >
              {{ index + 1 }}
            </span>
            <span class="hidden md:inline truncate">{{ step.short }}</span>
          </button>
        </li>
      </ol>
    </header>

    <div class="flex-1 grid grid-cols-1 lg:grid-cols-[minmax(0,1fr)_320px] gap-6 lg:gap-10 px-6 lg:px-10 py-8 lg:py-10">
      <div class="min-w-0">
        <Transition :name="direction === 'forward' ? 'step-forward' : 'step-backward'" mode="out-in">
          <PanelEventRegistrationStepAgreements
            v-if="currentStep === 'agreements'"
            ref="agreementsStep"
            key="agreements"
            v-model:agreements="draft.agreements"
            :is-underage="draft.personal.isUnderage"
          />
          <PanelEventRegistrationStepPersonal
            v-else-if="currentStep === 'personal'"
            ref="personalStep"
            key="personal"
            v-model:personal="draft.personal"
          />
          <PanelEventRegistrationStepTeam
            v-else-if="currentStep === 'team'"
            key="team"
            v-model:team="draft.team"
          />
          <PanelEventRegistrationStepConfirmation
            v-else
            ref="confirmationStep"
            key="confirmation"
            v-model:optional="draft.optional"
            v-model:personal="draft.personal"
            v-model:team="draft.team"
          />
        </Transition>
      </div>

      <PanelEventRegistrationStepShellSidebar :step="currentStep" />
    </div>

    <footer class="border-t-2 border-surface-muted px-6 lg:px-10 py-4 flex items-center justify-between gap-4 bg-surface-muted/20">
      <UButton
        type="button"
        color="neutral"
        variant="ghost"
        class="text-sm font-bold uppercase tracking-wider text-muted hover:text-default"
        :disabled="submitting"
        @click="prev"
      >
        {{ stepIndex === 0 ? 'Wróć do konta' : 'Wstecz' }}
      </UButton>

      <PanelActionButton
        v-if="stepIndex < STEPS.length - 1"
        filled
        :disabled="validating"
        @click="next"
      >
        {{ validating ? 'Sprawdzam...' : 'Dalej' }}
      </PanelActionButton>
      <PanelActionButton
        v-else
        tone="success"
        filled
        :disabled="submitting || validating"
        @click="onSubmit"
      >
        {{ submitting ? 'Wysyłam...' : validating ? 'Sprawdzam...' : 'Zarejestruj się' }}
      </PanelActionButton>
    </footer>
  </div>
</template>

<style scoped>
.step-forward-enter-active,
.step-forward-leave-active,
.step-backward-enter-active,
.step-backward-leave-active {
  transition:
    opacity 160ms ease,
    transform 200ms cubic-bezier(0.22, 1, 0.36, 1);
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
