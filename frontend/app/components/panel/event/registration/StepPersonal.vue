<script setup lang="ts">
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'
import { z } from 'zod'

const personal = defineModel<EventRegistrationDraft['personal']>('personal', { required: true })

const FOOD_OPTIONS = [
  { value: 'standard' as const, label: 'Standard', hint: 'Mięso i ryba' },
  { value: 'vegetarian' as const, label: 'Wegetariańskie', hint: 'Bez mięsa' },
]

const schema = z.object({
  fullName: z.string().min(2, 'Podaj imię i nazwisko'),
  school: z.string().min(3, 'Podaj nazwę szkoły'),
  birthYear: z.string()
    .regex(/^\d{4}$/, 'Podaj rok w formacie RRRR')
    .refine((y) => {
      const n = Number.parseInt(y)

      return n >= 1990 && n <= 2015
    }, 'Nieprawidłowy rok urodzenia'),
  phone: z.e164('Nieprawidłowy numer telefonu'),
  isUnderage: z.boolean(),
  foodPreference: z.union([z.literal('standard'), z.literal('vegetarian'), z.literal('')])
    .refine(Boolean, 'Wybierz preferencję żywieniową'),
  emergencyContactName: z.string(),
  emergencyContactPhone: z.string(),
  emergencyContactEmail: z.union([z.string().email('Nieprawidłowy adres e-mail'), z.literal('')]),
}).superRefine((value, ctx) => {
  if (!value.isUnderage)
    return

  if (!value.emergencyContactName.trim()) {
    ctx.addIssue({
      code: 'custom',
      path: ['emergencyContactName'],
      message: 'Podaj imię i nazwisko opiekuna',
    })
  }

  if (!value.emergencyContactPhone.trim()) {
    ctx.addIssue({
      code: 'custom',
      path: ['emergencyContactPhone'],
      message: 'Podaj telefon opiekuna',
    })
  }
})

const form = useTemplateRef('form')

async function validate() {
  await form.value?.validate({})
}

defineExpose({ validate })
</script>

<template>
  <UForm ref="form" :schema="schema" :state="personal" :validate-on="['blur', 'change']">
    <section class="space-y-7">
      <header>
        <h2 class="font-pixelify text-3xl lg:text-4xl leading-none">
          Kim <span class="text-primary">jesteś?</span>
        </h2>
        <p class="text-sm text-muted mt-3 max-w-xl leading-relaxed">
          Te dane pomagają nam przygotować identyfikator i wyżywienie.
        </p>
      </header>

      <div class="grid sm:grid-cols-2 gap-4">
        <UFormField label="Imię i nazwisko" name="fullName" required class="sm:col-span-2">
          <UInput
            v-model="personal.fullName"
            placeholder="np. Wanda Krakowiak"
            size="lg"
            class="w-full"
          />
        </UFormField>

        <UFormField label="Pełna nazwa szkoły" name="school" required class="sm:col-span-2">
          <UInput
            v-model="personal.school"
            placeholder="np. XXXI Liceum Ogólnokształcące im. Romana Ingardena w Krakowie"
            size="lg"
            class="w-full"
          />
        </UFormField>

        <UFormField label="Rok urodzenia" name="birthYear" required>
          <UInput
            v-model="personal.birthYear"
            placeholder="np. 2007"
            size="lg"
            maxlength="4"
            class="w-full"
          />
        </UFormField>

        <UFormField label="Telefon kontaktowy" name="phone" required>
          <UInput
            v-model="personal.phone"
            placeholder="+48123435678"
            size="lg"
            type="tel"
            class="w-full"
          />
        </UFormField>
      </div>

      <div>
        <p class="text-xs uppercase tracking-wider text-muted font-bold mb-3">
          Czy jesteś pełnoletni/a?
        </p>
        <div class="grid grid-cols-2 gap-2">
          <PanelOptionCard
            title="Nie, mam mniej niż 18 lat"
            description="Kontakt do opiekuna jest wymagany."
            :selected="personal.isUnderage"
            @select="personal.isUnderage = true"
          />
          <PanelOptionCard
            title="Tak, jestem pełnoletni/a"
            description="Kontakt awaryjny opcjonalny."
            :selected="!personal.isUnderage"
            @select="personal.isUnderage = false"
          />
        </div>
      </div>

      <div class="border-2 border-surface-muted">
        <header class="px-4 py-3 border-b-2 border-surface-muted bg-surface-muted/30">
          <h3
            class="text-sm font-bold uppercase tracking-wider"
            :class="personal.isUnderage ? 'text-primary' : 'text-muted'"
          >
            {{ personal.isUnderage ? 'Rodzic lub opiekun prawny' : 'Kontakt awaryjny' }}
            <span v-if="personal.isUnderage" class="text-error ml-0.5">*</span>
          </h3>
          <p class="text-xs text-muted mt-1 normal-case tracking-normal font-normal">
            {{ personal.isUnderage ? 'Wymagane dla osób niepełnoletnich.' : 'Opcjonalne · do kogo się zwrócić w razie sytuacji kryzysowej.' }}
          </p>
        </header>
        <div class="p-4 grid sm:grid-cols-2 gap-4">
          <UFormField label="Imię i nazwisko" name="emergencyContactName" class="sm:col-span-2" :required="personal.isUnderage">
            <UInput
              v-model="personal.emergencyContactName"
              placeholder="np. Anna Kowalska"
              size="lg"
              class="w-full"
            />
          </UFormField>
          <UFormField label="Telefon" name="emergencyContactPhone" :required="personal.isUnderage">
            <UInput
              v-model="personal.emergencyContactPhone"
              placeholder="+48123435678"
              size="lg"
              type="tel"
              class="w-full"
            />
          </UFormField>
          <UFormField label="Email" name="emergencyContactEmail">
            <UInput
              v-model="personal.emergencyContactEmail"
              placeholder="smok@krakow.pl"
              size="lg"
              type="email"
              class="w-full"
            />
          </UFormField>
        </div>
      </div>

      <UFormField label="Wyżywienie" name="foodPreference" required>
        <p class="text-xs text-muted mb-3 -mt-1">
          Wybierz preferowaną opcję posiłków podczas wydarzenia.
        </p>
        <div class="grid grid-cols-2 gap-2">
          <PanelOptionCard
            v-for="option in FOOD_OPTIONS"
            :key="option.value"
            :title="option.label"
            :description="option.hint"
            :selected="personal.foodPreference === option.value"
            @select="personal.foodPreference = option.value"
          />
        </div>
      </UFormField>
    </section>
  </UForm>
</template>
