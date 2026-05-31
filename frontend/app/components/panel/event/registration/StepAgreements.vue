<script setup lang="ts">
import type { FormError } from '@nuxt/ui'
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'
import { EVENT_DOCUMENTS } from '~~/content/event-documents'

const props = defineProps<{
  isUnderage: boolean
}>()

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
    title: 'Przyniosę podpisane zgody na wydarzenie',
    meta: 'Dwa formularze do pobrania poniżej · podpisane dokumenty pokażesz przy wejściu',
    badge: 'entrance',
    required: true,
  },
]

function badgeColor(badge: ConsentItem['badge']) {
  switch (badge) {
    case 'online':
      return 'success'
    case 'entrance':
      return 'primary'
    case 'optional':
      return 'neutral'
  }
}

function badgeLabel(badge: ConsentItem['badge']) {
  return badge === 'online' ? 'Online' : badge === 'entrance' ? 'Przy wejściu' : 'Opcjonalne'
}

function validateAgreements(state: Partial<EventRegistrationDraft['agreements']>): FormError[] {
  return CONSENTS
    .filter(item => item.required && (item.key !== 'parental' || props.isUnderage) && !state[item.key])
    .map(item => ({ name: item.key, message: 'Ta zgoda jest wymagana' }))
}

const form = useTemplateRef('form')

async function validate() {
  await form.value?.validate({})
}

defineExpose({ validate })
</script>

<template>
  <UForm ref="form" :state="agreements" :validate="validateAgreements" :validate-on="['blur', 'change']">
    <section class="space-y-6">
      <header>
        <h2 class="font-pixelify text-3xl lg:text-4xl leading-none">
          Najpierw <span class="text-primary">formalności.</span>
        </h2>
        <p class="text-sm text-muted mt-3 max-w-xl leading-relaxed">
          Zaznacz zgody online, pobierz dwa dokumenty do podpisu i przynieś je na wydarzenie.
        </p>
      </header>

      <div class="grid sm:grid-cols-2 gap-3">
        <NuxtLink
          v-for="document in EVENT_DOCUMENTS"
          :key="document.href"
          :href="document.href"
          download
          external
          class="border-2 border-primary/70 px-4 py-3 hover:bg-primary/5 transition-colors flex items-start gap-3"
        >
          <UIcon name="pixelarticons:download" class="size-5 text-primary mt-0.5 shrink-0" />
          <span>
            <span class="block text-sm font-bold">{{ document.title }}</span>
            <span class="block text-xs text-muted mt-1 leading-snug">{{ document.description }}</span>
          </span>
        </NuxtLink>
      </div>

      <ul class="space-y-3">
        <UFormField
          v-for="item in CONSENTS"
          :key="item.key"
          :name="item.key"
          as="li"
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
                <span v-if="item.required && (item.key !== 'parental' || isUnderage)" class="text-error ml-1">*</span>
              </p>
              <p class="text-xs text-muted mt-1 leading-snug">
                {{ item.meta }}
              </p>
            </div>

            <UBadge
              :color="badgeColor(item.badge)"
              variant="outline"
              size="sm"
              class="shrink-0 self-start"
            >
              {{ badgeLabel(item.badge) }}
            </UBadge>
          </label>
        </UFormField>
      </ul>
    </section>
  </UForm>
</template>
