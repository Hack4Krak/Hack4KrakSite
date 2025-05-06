<script setup lang="ts">
import type { FormSubmitEvent } from '#ui/types'
import { FetchError } from 'ofetch'
import * as z from 'zod'

definePageMeta({
  layout: 'centered',
})

type Schema = z.output<typeof schema>

const schema = z.object({
  first_name: z.string({ error: 'Imię jest wymagane' }),

  age: z.enum(
    ['10-13', '14-16', '16-18', '19-22', '23+'],
    { message: 'Wiek jest wymagany' },
  ),

  location: z.string({ error: 'Lokalizacja jest wymagana' }),

  organization: z.string({ error: 'Organizacja jest wymagana' }),

  is_vegetarian: z.boolean({ error: 'Wybór jest wymagany' }),

  marketing_consent: z.boolean({ error: 'Wybór jest wymagany' }),

  referral_source: z
    .array(z.enum([
      'Linkedin',
      'Facebook',
      'Instagram',
      'Znajomy',
      'Internet',
      'Inne',
    ]))
    .min(1, 'Proszę wybrać co najmniej jedno źródło polecenia.'),
})

const loading = ref(false)
const toast = useToast()
const state = reactive<Partial<Schema>>({
  first_name: undefined,
  age: undefined,
  location: undefined,
  organization: undefined,
  is_vegetarian: false,
  marketing_consent: false,
  referral_source: [],
})

const { data } = await useAuth('/account/get_personal_information')

const current_year = new Date().getFullYear()

if (data.value) {
  state.first_name = data.value.first_name

  switch (data.value.birth_year) {
    case current_year - 10:
      state.age = '10-13'
      break
    case current_year - 14:
      state.age = '14-16'
      break
    case current_year - 16:
      state.age = '16-18'
      break
    case current_year - 19:
      state.age = '19-22'
      break
    case current_year - 23:
      state.age = '23+'
      break
  }

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

  let birth_year

  switch (event.data.age) {
    case '10-13':
      birth_year = current_year - 10
      break
    case '14-16':
      birth_year = current_year - 14
      break
    case '16-18':
      birth_year = current_year - 16
      break
    case '19-22':
      birth_year = current_year - 19
      break
    case '23+':
      birth_year = current_year - 23
      break
  }

  try {
    await useNuxtApp().$auth('/account/submit_personal_information', {
      method: 'POST',
      credentials: 'include',
      body: {
        first_name: event.data.first_name,
        birth_year,
        location: event.data.location,
        organization: event.data.organization,
        is_vegetarian: event.data.is_vegetarian ?? false,
        marketing_consent: event.data.marketing_consent ?? false,
        referral_source: event.data.referral_source,
      },
    })

    toast.add({ title: 'Sukces', description: 'Pomyślnie uzupełniono dane', color: 'success' })

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
  <div class="w-170">
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
          v-model="state.age" :items="['10-13', '14-16', '16-18', '19-22', '23+']"
          class="w-full"
        />
      </UFormField>
      <UFormField label="Lokalizacja" name="location">
        <TransparentInput v-model="state.location" />
      </UFormField>
      <UFormField label="Organizacja" name="organization">
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
          v-model="state.referral_source" :items="['Linkedin', 'Facebook', 'Instagram', 'Znajomy', 'Internet', 'Inne']"
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
