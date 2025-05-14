<script setup lang="ts">
import type { FormSubmitEvent } from '#ui/types'
import { FetchError } from 'ofetch'
import * as z from 'zod'

definePageMeta({
  layout: 'centered',
})

const REFERRAL_SOURCES = [
  'Linkedin',
  'Facebook',
  'Instagram',
  'Znajomy',
  'Internet',
  'Inne',
]

const currentYear = new Date().getFullYear()

const loading = ref(false)

enum Age {
  '10-13' = 10,
  '14-16' = 14,
  '16-18' = 16,
  '19-22' = 19,
  '23+' = 23,
}

function matchAge(age: Age) {
  return currentYear - age
}

function getAge(birth_year: number): string | undefined {
  const age = currentYear - birth_year
  for (const [key, value] of Object.entries(Age).reverse()) {
    if (age >= Number(value)) {
      return key
    }
  }
  return undefined
}

type Schema = z.output<typeof schema>

const ageKeys = Object.keys(Age).filter(key => Number.isNaN(Number(key)))

const schema = z.object({
  first_name: z.string({ error: 'Imię jest wymagane' }),
  age: z.enum(
    ageKeys,
    { message: 'Wiek jest wymagany' },
  ),
  location: z.string({ error: 'Lokalizacja jest wymagana' }),
  organization: z.string({ error: 'Organizacja jest wymagana' }),
  is_vegetarian: z.boolean({ error: 'Wybór jest wymagany' }),
  marketing_consent: z.boolean().optional(),
  referral_source: z
    .array(z.enum(REFERRAL_SOURCES))
    .min(1, 'Proszę wybrać co najmniej jedno źródło polecenia.'),
})

const state = reactive<Partial<Schema>>({
  first_name: undefined,
  age: undefined,
  location: undefined,
  organization: undefined,
  is_vegetarian: false,
  marketing_consent: false,
  referral_source: [],
})

const { data } = await useAuth('/account/get_personal_information', {
  onResponseError: undefined,
})

if (data.value) {
  state.first_name = data.value.first_name
  state.age = getAge(data.value.birth_year)
  state.location = data.value.location
  state.organization = data.value.organization
  state.is_vegetarian = data.value.is_vegetarian
  state.marketing_consent = data.value.marketing_consent

  if (typeof data.value.referral_source === typeof state.referral_source) {
    state.referral_source = data.value?.referral_source as any
  }
}

const isButtonEnabled = computed(() => {
  return schema.safeParse(state).success
})

async function onSubmit(event: FormSubmitEvent<Schema>) {
  event.preventDefault()

  loading.value = true

  const birth_year = matchAge(Age[event.data.age as keyof typeof Age])

  try {
    await useNuxtApp().$auth('/account/submit_personal_information', {
      method: 'POST',
      credentials: 'include',
      body: {
        first_name: event.data.first_name,
        birth_year,
        location: event.data.location,
        organization: event.data.organization,
        is_vegetarian: event.data.is_vegetarian,
        marketing_consent: event.data.marketing_consent ?? false,
        referral_source: event.data.referral_source,
      },
    })

    useToast().add({ title: 'Sukces', description: 'Pomyślnie uzupełniono dane', color: 'success' })

    await refreshNuxtData()
    navigateTo('/panel')
  } catch (error) {
    console.error(error)
    if (!(error instanceof FetchError)) {
      throw error
    }
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="md:w-170 sm:w-90 w-60">
    <h1 class="text-2xl font-medium mb-2">
      Podaj dodatkowe informacje
    </h1>

    <div class="text-sm mb-4 flex flex-col gap-3">
      <p>
        Zanim zaczniesz korzystać ze strony, podaj proszę dodatkowe informacje o koncie
      </p>
      <p>
        Dane, które tu udostępnisz, będą widoczne jedynie dla organizatorów wydarzenia i są konieczne do jego przeprowadzenia.
      </p>
    </div>

    <UForm :schema="schema" :state="state" class="space-y-4 text-center" @submit="onSubmit">
      <UFormField label="Imię" name="first_name">
        <TransparentInput v-model="state.first_name" />
      </UFormField>
      <UFormField label="Wiek" name="age">
        <USelect
          v-model="state.age" :items="ageKeys"
          class="w-full"
        />
      </UFormField>
      <UFormField label="Lokalizacja" name="location">
        <template #description>
          <p class="text-dimmed text-left">
            Miasto, w którym mieszkasz
          </p>
        </template>
        <TransparentInput v-model="state.location" />
      </UFormField>
      <UFormField label="Organizacja" name="organization">
        <template #description>
          <p class="text-dimmed text-left">
            Twoja szkoła, uczelnia lub firma
          </p>
        </template>
        <TransparentInput v-model="state.organization" />
      </UFormField>
      <UFormField label="Preferencje żywieniowe" name="is_vegetarian">
        <USelect
          v-model="state.is_vegetarian" :items="[
            {
              label: 'Dieta mięsna',
              value: false,
            },
            {
              label: 'Dieta wegańska/wegetariańska',
              value: true,
            },
          ]"
          class="w-full"
        />
      </UFormField>
      <UFormField label="Czy chcesz dostawać informacje o kolejnych wydarzeniach organizowanych przez Hack4Krak?" name="marketing_consent" class="flex items-center justify-between text-left">
        <UCheckbox v-model="state.marketing_consent" :default-value="false" />
      </UFormField>
      <UFormField label="Skąd się o nas dowiedziałeś? (wielokrotny wybór)" name="referral_source" class="flex-col items-center">
        <USelect
          v-model="state.referral_source" :items="REFERRAL_SOURCES"
          multiple
          class="w-full"
        />
      </UFormField>

      <div class="space-y-2">
        <UButton type="submit" class="w-full text-center inline rounded-3xl py-2 bg-neutral-300" :disabled="loading" :class="isButtonEnabled ? 'bg-(--ui-primary)' : ''">
          Wyślij
        </UButton>
      </div>
    </UForm>
  </div>
</template>
