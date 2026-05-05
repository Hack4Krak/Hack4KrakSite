<script setup lang="ts">
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'

const optional = defineModel<EventRegistrationDraft['optional']>('optional', { required: true })
const personal = defineModel<EventRegistrationDraft['personal']>('personal', { required: true })
const team = defineModel<EventRegistrationDraft['team']>('team', { required: true })

const summaryRows = computed(() => {
  const rows = [
    { label: 'Uczestnik', value: personal.value.fullName || '—' },
    { label: 'Szkoła', value: personal.value.school || '—' },
    { label: 'Rok urodzenia', value: personal.value.birthYear || '—' },
    { label: 'Drużyna', value: team.value.teamName || '— (nie utworzona)' },
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
}

const COMMITMENTS: Commitment[] = [
  {
    key: 'confirmAttendance',
    title: 'Potwierdzam, że się stawię lub poinformuję organizatorów',
    hint: 'Pomaga nam zaplanować logistykę.',
  },
  {
    key: 'bringOwnLaptop',
    title: 'Przyniosę własny laptop lub sprzęt do rozwiązywania',
    hint: 'Mamy gniazdka pod każdym stanowiskiem.',
  },
  {
    key: 'bringDocuments',
    title: 'Przyniosę zgody oraz dokument tożsamości',
    hint: 'Wydrukowane formularze z linka na wydarzenie · do okazania przy wejściu.',
  },
]
</script>

<template>
  <section class="space-y-7">
    <header>
      <h2 class="font-pixelify text-3xl lg:text-4xl leading-none">
        Ostatni <span class="text-primary">krok.</span>
      </h2>
      <p class="text-sm text-muted mt-3 max-w-xl leading-relaxed">
        Sprawdź dane i potwierdź, że jesteś gotowy/a na dzień eventu.
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
      <p class="text-xs text-muted -mt-1">
        Wszystkie trzy są wymagane do wysłania zgłoszenia. Nie są przechowywane w bazie danych.
      </p>

      <label
        v-for="item in COMMITMENTS"
        :key="item.key"
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
            <UIcon v-if="optional[item.key]" name="pixelarticons:check" class="size-4 text-default" />
          </span>
        </span>
        <div class="flex-1">
          <p class="text-sm leading-snug font-medium">
            {{ item.title }} <span class="text-error">*</span>
          </p>
          <p class="text-xs text-muted mt-1">
            {{ item.hint }}
          </p>
        </div>
      </label>
    </div>

    <div class="border-2 border-success/40 bg-success/5 px-4 py-3 flex items-start gap-3">
      <UIcon name="pixelarticons:flag" class="size-5 text-success mt-0.5 shrink-0" />
      <p class="text-xs text-success leading-relaxed">
        Po wysłaniu otrzymasz mailem QR kod uczestnika. Ten sam kod znajdziesz w panelu wydarzenia.
      </p>
    </div>
  </section>
</template>
