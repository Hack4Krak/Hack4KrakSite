<script setup lang="ts">
useSeoMeta({
  title: 'Rejestracja na Hack4Krak 2026',
  description: 'Zapisz drużynę na nadchodzącą edycję Hack4Krak CTF.',
})

const {
  draft,
  draftReady,
  isRegistered,
  hasTeam,
  pending,
  submitRegistration,
  reset,
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
const submitting = ref(false)

const currentStep = computed<StepId>(() => STEPS[stepIndex.value]!.id)

const stepValid = computed<boolean[]>(() => {
  const personal = draft.value.personal
  const personalDone = personal.fullName.trim()
    && personal.school.trim()
    && personal.birthYear.trim()
    && personal.phone.trim()
    && personal.foodPreference
    && (!personal.isUnderage || (personal.emergencyContactName.trim() && personal.emergencyContactPhone.trim()))

  const { confirmAttendance, bringOwnLaptop, bringDocuments } = draft.value.optional

  return [
    draft.value.agreements.rules
    && draft.value.agreements.rodo
    && (!draft.value.personal.isUnderage || draft.value.agreements.parental),
    Boolean(personalDone),
    draft.value.team.confirmed,
    confirmAttendance && bringOwnLaptop && bringDocuments,
  ]
})

const canAdvance = computed(() => stepValid.value[stepIndex.value])
function next() {
  if (!canAdvance.value)
    return
  if (stepIndex.value < STEPS.length - 1) {
    direction.value = 'forward'
    stepIndex.value++
  }
}

function prev() {
  if (stepIndex.value === 0) {
    return navigateTo('/panel/profile')
  }
  direction.value = 'backward'
  stepIndex.value--
}

async function onSubmit() {
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
  <div class="panel-page">
    <div class="m-4 lg:m-8 border-2 border-surface-muted bg-default min-h-[calc(100vh-12rem)] flex flex-col">
      <header>
        <div class="px-6 lg:px-10 pt-6 pb-4 flex items-center gap-4 flex-wrap">
          <div class="flex items-baseline gap-3">
            <span class="text-xs uppercase tracking-wider text-muted">
              Krok <span class="text-primary font-bold">{{ stepIndex + 1 }}</span> z {{ STEPS.length }}
            </span>
            <span class="hidden sm:inline text-xs uppercase tracking-wider text-default font-bold">
              · {{ STEPS[stepIndex]!.label }}
            </span>
          </div>
        </div>

        <div class="grid grid-cols-4 gap-px bg-surface-muted border-y-2 border-surface-muted">
          <button
            v-for="(step, index) in STEPS"
            :key="step.id"
            type="button"
            :disabled="index > stepIndex && !stepValid.slice(0, index).every(Boolean)"
            class="bg-default px-3 py-2.5 text-xs uppercase tracking-wider font-bold transition-all flex items-center justify-center gap-2 cursor-pointer disabled:cursor-not-allowed border-b-2"
            :class="index === stepIndex
              ? 'text-primary border-primary bg-primary/5'
              : index < stepIndex
                ? 'text-default border-primary/40 hover:bg-surface-muted/30'
                : 'text-muted/50 border-transparent'"
            @click="stepIndex = index; direction = index > stepIndex ? 'forward' : 'backward'"
          >
            <span
              class="size-5 flex items-center justify-center border-2 text-xs"
              :class="index < stepIndex
                ? 'bg-primary border-primary text-default'
                : index === stepIndex
                  ? 'border-primary text-primary'
                  : 'border-surface-muted text-muted/50'"
            >
              <UIcon v-if="index < stepIndex" name="pixelarticons:check" class="size-3.5" />
              <span v-else>{{ index + 1 }}</span>
            </span>
            <span class="hidden md:inline">{{ step.short }}</span>
          </button>
        </div>
      </header>

      <div class="flex-1 grid grid-cols-1 lg:grid-cols-[1fr_360px] gap-4 lg:gap-6 p-4 lg:p-6">
        <div class="p-6 lg:p-10 overflow-y-auto">
          <Transition :name="direction === 'forward' ? 'step-forward' : 'step-backward'" mode="out-in">
            <PanelEventRegistrationStepAgreements
              v-if="currentStep === 'agreements'"
              key="agreements"
              v-model:agreements="draft.agreements"
            />
            <PanelEventRegistrationStepPersonal
              v-else-if="currentStep === 'personal'"
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
        <button
          type="button"
          class="text-sm font-bold uppercase tracking-wider text-muted hover:text-default flex items-center gap-2 transition-colors cursor-pointer"
          :disabled="submitting"
          @click="prev"
        >
          <UIcon name="pixelarticons:arrow-left" class="size-4" />
          {{ stepIndex === 0 ? 'Wróć do konta' : 'Wstecz' }}
        </button>

        <button
          v-if="stepIndex < STEPS.length - 1"
          type="button"
          class="text-sm font-bold uppercase tracking-wider px-5 py-2.5 bg-primary text-default hover:bg-secondary disabled:opacity-40 disabled:cursor-not-allowed transition-colors flex items-center gap-2 cursor-pointer"
          :disabled="!canAdvance"
          @click="next"
        >
          Dalej
          <UIcon name="pixelarticons:arrow-right" class="size-4" />
        </button>
        <button
          v-else
          type="button"
          class="text-sm font-bold uppercase tracking-wider px-5 py-2.5 bg-success text-default hover:opacity-90 disabled:opacity-40 disabled:cursor-not-allowed transition-opacity flex items-center gap-2 cursor-pointer"
          :disabled="submitting || !canAdvance"
          @click="onSubmit"
        >
          <UIcon
            :name="submitting ? 'pixelarticons:loader' : 'pixelarticons:flag'"
            class="size-4"
            :class="submitting ? 'animate-spin' : ''"
          />
          {{ submitting ? 'Wysyłam...' : 'Zarejestruj drużynę' }}
        </button>
      </footer>
    </div>
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
