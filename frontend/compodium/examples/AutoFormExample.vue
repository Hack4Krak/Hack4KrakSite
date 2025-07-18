<script setup lang="ts">
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
</script>

<template>
  <AutoForm :schema="schema" @submit="data => console.log(data)">
    <template #is_vegetarian="{ field, state }">
      <USelect
        v-model="state[field]"
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
</template>
