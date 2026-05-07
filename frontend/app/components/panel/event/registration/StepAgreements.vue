<script setup lang="ts">
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'

const agreements = defineModel<EventRegistrationDraft['agreements']>('agreements', { required: true })

interface ConsentItem {
  key: keyof EventRegistrationDraft['agreements']
  title: string
  meta: string
  badge: 'online' | 'entrance' | 'optional'
  required: boolean
}

const CONSENTS: ConsentItem[] = [
  {
    key: 'rules',
    title: 'Zapoznałem/am się z regulaminem wydarzenia',
    meta: 'Pełna treść w sekcji Regulamin',
    badge: 'online',
    required: true,
  },
  {
    key: 'rodo',
    title: 'Wyrażam zgodę na przetwarzanie danych osobowych',
    meta: 'RODO · pełna treść w regulaminie',
    badge: 'online',
    required: true,
  },
  {
    key: 'parental',
    title: 'Rodzic/opiekun prawny wyraził zgodę na mój udział',
    meta: 'Deklaratywne online · papierowe potwierdzenie przy wejściu jeśli niepełnoletni/a',
    badge: 'entrance',
    required: true,
  },
  {
    key: 'bringConsents',
    title: 'Zobowiązuję się przynieść wszystkie zgody z linka na wydarzenie',
    meta: 'Wydrukowane formularze · do okazania przy wejściu',
    badge: 'entrance',
    required: false,
  },
]

function badgeClass(badge: ConsentItem['badge']) {
  switch (badge) {
    case 'online':
      return 'border-success text-success'
    case 'entrance':
      return 'border-primary text-primary'
    case 'optional':
      return 'border-muted text-muted'
  }
}

function badgeLabel(badge: ConsentItem['badge']) {
  return badge === 'online' ? 'online' : badge === 'entrance' ? 'przy wejściu' : 'opcjonalne'
}
</script>

<template>
  <section class="space-y-6">
    <header>
      <h2 class="font-pixelify text-3xl lg:text-4xl leading-none">
        Najpierw <span class="text-primary">formalności.</span>
      </h2>
      <p class="text-sm text-muted mt-3 max-w-xl leading-relaxed">
        Trzy zgody są wymagane do kontynuowania. Czwarta to deklaracja przyniesienia fizycznych dokumentów.
      </p>
    </header>

    <ul class="space-y-3">
      <li
        v-for="item in CONSENTS"
        :key="item.key"
      >
        <label
          class="flex items-start gap-4 px-4 py-4 border-2 cursor-pointer transition-colors"
          :class="agreements[item.key]
            ? 'border-primary bg-primary/5'
            : 'border-surface-muted hover:border-primary/40'"
        >
          <span class="relative mt-0.5 shrink-0">
            <input
              v-model="agreements[item.key]"
              type="checkbox"
              class="peer sr-only"
            >
            <span
              class="size-5 border-2 flex items-center justify-center transition-colors"
              :class="agreements[item.key]
                ? 'border-primary bg-primary'
                : 'border-surface-muted'"
            >
              <span v-if="agreements[item.key]" class="size-2 bg-default" />
            </span>
          </span>

          <div class="flex-1 min-w-0">
            <p class="text-sm leading-snug font-medium">
              {{ item.title }}
              <span v-if="item.required" class="text-error ml-1">*</span>
            </p>
            <p class="text-xs text-muted mt-1 leading-snug">
              {{ item.meta }}
            </p>
          </div>

          <span
            class="text-xs font-bold uppercase tracking-wider border px-2 py-1 shrink-0 self-start"
            :class="badgeClass(item.badge)"
          >
            {{ badgeLabel(item.badge) }}
          </span>
        </label>
      </li>
    </ul>
  </section>
</template>
