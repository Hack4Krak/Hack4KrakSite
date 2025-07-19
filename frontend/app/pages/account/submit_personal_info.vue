<script setup lang="ts">
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

enum Age {
  '10-13' = 10,
  '14-16' = 14,
  '16-18' = 16,
  '19-22' = 19,
  '23+' = 23,
}

const currentYear = new Date().getFullYear()
const ageKeys = Object.keys(Age).filter(key => Number.isNaN(Number(key)))

const schema = z.object({
  first_name: z.string()
    .nonempty({ error: 'Imię jest wymagane' })
    .meta({ title: 'Imię' }),
  age: z.enum(ageKeys, { message: 'Wiek jest wymagany' })
    .meta({ title: 'Wiek' }),
  location: z.string({ error: 'Lokalizacja jest wymagana' })
    .meta({ title: 'Lokalizacja', description: 'Miasto, w którym mieszkasz' }),
  organization: z.string({ error: 'Organizacja jest wymagana' })
    .meta({ title: 'Organizacja', description: 'Twoja szkoła, uczelnia lub firma' }),
  is_vegetarian: z.boolean({ error: 'Wybór jest wymagany' })
    .meta({ title: 'Preferencje żywieniowe' }),
  marketing_consent: z.boolean()
    .meta({
      title: 'Czy chcesz dostawać informacje o kolejnych wydarzeniach organizowanych przez Hack4Krak?',
      autoForm: { floatRight: true },
    }),
  referral_source: z
    .array(z.enum(REFERRAL_SOURCES))
    .min(1, 'Proszę wybrać co najmniej jedno źródło polecenia.')
    .meta({ title: 'Skąd się o nas dowiedziałeś? (wielokrotny wybór)' }),
})

const { data } = await useAuth('/account/get_personal_information', {
  onResponseError: undefined,
})

const state = data.value
  ? {
      referral_source: data.value.referral_source as any,
      age: getAge(data.value.birth_year),
      ...data.value,
    }
  : {}

// Helper function to convert birth year to age category
function getAge(birth_year: number): string | undefined {
  const age = currentYear - birth_year
  for (const [key, value] of Object.entries(Age).reverse()) {
    if (age >= Number(value)) {
      return key
    }
  }
  return undefined
}

async function onSubmit(data: zInfer<typeof schema>) {
  const birth_year = currentYear - Age[data.age as keyof typeof Age]

  await useNuxtApp().$auth('/account/submit_personal_information', {
    method: 'POST',
    credentials: 'include',
    body: {
      ...data,
      birth_year,
    },
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie uzupełniono dane', color: 'success' })

  await refreshNuxtData()
  navigateTo('/panel')
}
</script>

<template>
  <div class="md:w-170 sm:w-90 w-60">
    <h1 class="text-2xl font-medium mb-2">
      Podaj dodatkowe informacje
    </h1>

    <div class="text-sm mb-4 space-y-3">
      <p>
        Zanim zaczniesz korzystać ze strony, podaj proszę dodatkowe informacje o koncie
      </p>
      <p>
        Dane, które tu udostępnisz, będą widoczne jedynie dla organizatorów wydarzenia i są konieczne do jego przeprowadzenia.
      </p>
    </div>

    <AutoForm :schema="schema" :initial-state="state" @submit="onSubmit">
      <template #is_vegetarian="{ field, state: stateValue }">
        <USelect
          v-model="stateValue[field]"
          :items="[
            {
              label: 'Dieta mięsna',
              value: false,
            },
            {
              label: 'Dieta wegańska/wegetariańska',
              value: true,
            },
          ]"
        />
      </template>
    </AutoForm>
  </div>
</template>
