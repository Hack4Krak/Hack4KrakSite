<script setup lang="ts">
import type { EventRegistrationDraft } from '~/composables/useEventRegistration'

const personal = defineModel<EventRegistrationDraft['personal']>('personal', { required: true })

interface Option<T extends string> {
  value: T
  label: string
  hint?: string
}

const FOOD_OPTIONS: Option<EventRegistrationDraft['personal']['foodPreference']>[] = [
  { value: 'standard', label: 'Standard', hint: 'Mięso i ryba' },
  { value: 'vegetarian', label: 'Wegetariańskie', hint: 'Bez mięsa' },
]
</script>

<template>
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
      <label class="flex flex-col gap-2 sm:col-span-2">
        <span class="text-xs uppercase tracking-wider text-muted font-bold">
          Imię i nazwisko <span class="text-error ml-0.5">*</span>
        </span>
        <UInput
          v-model="personal.fullName"
          placeholder="np. Jan Kowalski"
          size="lg"
          :ui="{ base: 'rounded-none' }"
        />
      </label>

      <label class="flex flex-col gap-2 sm:col-span-2">
        <span class="text-xs uppercase tracking-wider text-muted font-bold">
          Pełna nazwa szkoły <span class="text-error ml-0.5">*</span>
        </span>
        <UInput
          v-model="personal.school"
          placeholder="np. XIV Liceum Ogólnokształcące im. Stanisława Staszica w Krakowie"
          size="lg"
          :ui="{ base: 'rounded-none' }"
        />
      </label>

      <label class="flex flex-col gap-2">
        <span class="text-xs uppercase tracking-wider text-muted font-bold">
          Rok urodzenia <span class="text-error ml-0.5">*</span>
        </span>
        <UInput
          v-model="personal.birthYear"
          placeholder="np. 2007"
          size="lg"
          :ui="{ base: 'rounded-none' }"
        />
      </label>

      <label class="flex flex-col gap-2">
        <span class="text-xs uppercase tracking-wider text-muted font-bold">
          Telefon kontaktowy <span class="text-error ml-0.5">*</span>
        </span>
        <UInput
          v-model="personal.phone"
          placeholder="+48 xxx xxx xxx"
          size="lg"
          :ui="{ base: 'rounded-none' }"
        />
      </label>
    </div>

    <div>
      <p class="text-xs uppercase tracking-wider text-muted font-bold mb-3">
        Czy jesteś niepełnoletni/a?
      </p>
      <div class="grid grid-cols-2 gap-2">
        <button
          type="button"
          class="border-2 px-4 py-3 transition-colors cursor-pointer text-left"
          :class="personal.isUnderage
            ? 'border-primary bg-primary/5'
            : 'border-surface-muted hover:border-primary/40'"
          @click="personal.isUnderage = true"
        >
          <span
            class="block text-sm font-bold"
            :class="personal.isUnderage ? 'text-primary' : 'text-default'"
          >
            Tak, mam mniej niż 18 lat
          </span>
          <span class="block text-xs text-muted mt-1">
            Kontakt do opiekuna jest wymagany.
          </span>
        </button>
        <button
          type="button"
          class="border-2 px-4 py-3 transition-colors cursor-pointer text-left"
          :class="!personal.isUnderage
            ? 'border-primary bg-primary/5'
            : 'border-surface-muted hover:border-primary/40'"
          @click="personal.isUnderage = false"
        >
          <span
            class="block text-sm font-bold"
            :class="!personal.isUnderage ? 'text-primary' : 'text-default'"
          >
            Nie, jestem pełnoletni/a
          </span>
          <span class="block text-xs text-muted mt-1">
            Kontakt awaryjny opcjonalny.
          </span>
        </button>
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
        <label class="flex flex-col gap-2 sm:col-span-2">
          <span class="text-xs uppercase tracking-wider text-muted font-bold">
            Imię i nazwisko
            <span v-if="personal.isUnderage" class="text-error ml-0.5">*</span>
          </span>
          <UInput
            v-model="personal.emergencyContactName"
            placeholder="np. Anna Kowalska"
            size="lg"
            :ui="{ base: 'rounded-none' }"
          />
        </label>
        <label class="flex flex-col gap-2">
          <span class="text-xs uppercase tracking-wider text-muted font-bold">
            Telefon
            <span v-if="personal.isUnderage" class="text-error ml-0.5">*</span>
          </span>
          <UInput
            v-model="personal.emergencyContactPhone"
            placeholder="+48 xxx xxx xxx"
            size="lg"
            :ui="{ base: 'rounded-none' }"
          />
        </label>
        <label class="flex flex-col gap-2">
          <span class="text-xs uppercase tracking-wider text-muted font-bold">
            Email
            <span class="text-muted/60 normal-case tracking-normal text-xs ml-1 font-normal">opcjonalnie</span>
          </span>
          <UInput
            v-model="personal.emergencyContactEmail"
            placeholder="mail@example.com"
            size="lg"
            :ui="{ base: 'rounded-none' }"
          />
        </label>
      </div>
    </div>

    <div>
      <p class="text-xs uppercase tracking-wider text-muted font-bold mb-1">
        Wyżywienie <span class="text-error ml-0.5">*</span>
      </p>
      <p class="text-xs text-muted mb-3">
        Oferujemy dwie opcje. Jeśli masz alergie, wpisz je poniżej.
      </p>
      <div class="grid grid-cols-2 gap-2">
        <button
          v-for="option in FOOD_OPTIONS"
          :key="option.value"
          type="button"
          class="border-2 px-4 py-3 flex flex-col items-start gap-1 transition-colors cursor-pointer text-left"
          :class="personal.foodPreference === option.value
            ? 'border-primary bg-primary/5'
            : 'border-surface-muted hover:border-primary/40'"
          @click="personal.foodPreference = option.value"
        >
          <span
            class="font-bold text-sm"
            :class="personal.foodPreference === option.value ? 'text-primary' : 'text-default'"
          >
            {{ option.label }}
          </span>
          <span class="text-xs text-muted">
            {{ option.hint }}
          </span>
        </button>
      </div>
    </div>

    <label class="flex flex-col gap-2">
      <span class="text-xs uppercase tracking-wider text-muted font-bold">
        Alergie / nietolerancje
        <span class="text-muted/60 normal-case tracking-normal text-xs ml-1 font-normal">opcjonalne</span>
      </span>
      <UTextarea
        v-model="personal.foodAllergies"
        placeholder="np. orzechy, laktoza..."
        :rows="2"
        :ui="{ base: 'rounded-none' }"
      />
    </label>
  </section>
</template>
