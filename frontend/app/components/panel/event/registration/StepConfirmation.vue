<script setup lang="ts">
import type { FormError } from '@nuxt/ui'
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'

const optional = defineModel<EventRegistrationDraft['optional']>('optional', { required: true })
const personal = defineModel<EventRegistrationDraft['personal']>('personal', { required: true })
const team = defineModel<EventRegistrationDraft['team']>('team', { required: true })

const summaryRows = computed(() => {
  const rows = [
    { label: 'Uczestnik', value: personal.value.fullName || '—' },
    { label: 'Szkoła', value: personal.value.school || '—' },
    { label: 'Rok urodzenia', value: personal.value.birthYear || '—' },
    { label: 'Drużyna', value: team.value.teamName || 'Solo - dobiorę skład później' },
  ]
  if (personal.value.emergencyContactName || personal.value.isUnderage) {
    rows.push({
      label: personal.value.isUnderage ? 'Opiekun' : 'Kontakt awaryjny',
      value: personal.value.emergencyContactName || '—',
    })
  }
  rows.push({ label: 'Wyżywienie', value: foodLabel(personal.value.foodPreference) })
  return rows
})

function foodLabel(value: EventRegistrationDraft['personal']['foodPreference']): string {
  switch (value) {
    case 'standard': return 'Standard'
    case 'vegetarian': return 'Wegetariańskie'
    default: return '—'
  }
}

interface Commitment {
  key: keyof EventRegistrationDraft['optional']
  title: string
  hint: string
  required: boolean
}

const COMMITMENTS: Commitment[] = [
  {
    key: 'confirmAttendance',
    title: 'Planuję uczestniczyć w całym wydarzeniu',
    hint: 'Hack4Krak trwa dwie doby. W razie nieprzewidzianych okoliczności powiadomię organizatorów jak najwcześniej.',
    required: true,
  },
  {
    key: 'bringOwnLaptop',
    title: 'Zabiorę sprzęt potrzebny do udziału w CTF (laptop z ładowarką oraz wszelkie inne urządzenia, które potrzebuję)',
    hint: 'Na miejscu zapewniamy dostęp do zasilania. Nie udostępniamy sprzętu komputerowego — każdy uczestnik musi mieć własny.',
    required: true,
  },
  {
    key: 'bringDocuments',
    title: 'Zabiorę dokument tożsamości i podpisane zgody',
    hint: 'Sprawdzimy dokumenty przy wejściu. Bez ich okazania wejście na wydarzenie może być niemożliwe.',
    required: true,
  },
  {
    key: 'bringBedding',
    title: 'Zabiorę śpiwór lub pościel',
    hint: 'Wydarzenie trwa dwie doby — na miejscu będą wyznaczone strefy odpoczynku. Pościel ani śpiwór nie są wymagane, ale warto zabrać.',
    required: false,
  },
]

function validateCommitments(state: Partial<EventRegistrationDraft['optional']>): FormError[] {
  return COMMITMENTS
    .filter(item => item.required && !state[item.key])
    .map(item => ({ name: item.key, message: 'To potwierdzenie jest wymagane' }))
}

const form = useTemplateRef('form')

async function validate() {
  await form.value?.validate({})
}

defineExpose({ validate })
</script>

<template>
  <UForm ref="form" :state="optional" :validate="validateCommitments" :validate-on="['blur', 'change']">
    <section class="space-y-7">
      <header>
        <h2 class="font-pixelify text-3xl lg:text-4xl leading-none">
          Ostatni <span class="text-primary">krok.</span>
        </h2>
        <p class="text-sm text-muted mt-3 max-w-xl leading-relaxed">
          Sprawdź dane i potwierdź najważniejsze rzeczy organizacyjne przed wysłaniem zgłoszenia.
        </p>
      </header>

      <div class="border-2 border-surface-muted">
        <header class="px-4 py-3 border-b-2 border-surface-muted bg-surface-muted/30">
          <p class="text-xs uppercase tracking-wider text-primary font-bold">
            Podsumowanie
          </p>
        </header>
        <ul class="divide-y-2 divide-surface-muted">
          <li
            v-for="row in summaryRows"
            :key="row.label"
            class="flex items-baseline justify-between gap-4 px-4 py-3 text-sm"
          >
            <span class="text-muted text-xs uppercase tracking-wider">{{ row.label }}</span>
            <span class="font-medium text-right truncate max-w-[60%]">{{ row.value }}</span>
          </li>
        </ul>
      </div>

      <div class="space-y-3">
        <p class="text-xs uppercase tracking-wider text-muted font-bold">
          Potwierdzenia <span class="text-error ml-0.5">*</span>
        </p>

        <UFormField
          v-for="item in COMMITMENTS"
          :key="item.key"
          :name="item.key"
        >
          <label
            class="flex items-start gap-3 px-4 py-3 border-2 cursor-pointer transition-colors"
            :class="optional[item.key]
              ? 'border-primary bg-primary/5'
              : 'border-surface-muted hover:border-primary/40'"
          >
            <span class="relative mt-0.5 shrink-0">
              <input v-model="optional[item.key]" type="checkbox" class="peer sr-only">
              <span
                class="size-5 border-2 flex items-center justify-center transition-colors"
                :class="optional[item.key]
                  ? 'border-primary bg-primary'
                  : 'border-surface-muted'"
              >
                <span v-if="optional[item.key]" class="size-2 bg-default" />
              </span>
            </span>
            <div class="flex-1">
              <p class="text-sm leading-snug font-medium">
                {{ item.title }}
                <span v-if="item.required" class="text-error">*</span>
                <span v-else class="text-muted/60 text-xs font-normal ml-1">opcjonalnie</span>
              </p>
              <p class="text-xs text-muted mt-1">
                {{ item.hint }}
              </p>
            </div>
          </label>
        </UFormField>
      </div>
    </section>
  </UForm>
</template>
