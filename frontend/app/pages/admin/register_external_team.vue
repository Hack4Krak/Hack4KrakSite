<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as z from 'zod'

const toast = useToast()
const route = useRoute()

const confirmationCode = String(route.query.code)

const { data, refresh } = await useApi('/teams/external_invitations/list/{code}', {
  path: {
    code: confirmationCode,
  },
})

const { data: school, error } = await useApi('/teams/external_invitations/info/{code}', {
  path: {
    code: confirmationCode,
  },
  onResponseError: undefined,
})

if (error.value) {
  showError({
    statusCode: 400,
    message: 'Zły kod rejestracji.\n Skontaktuj się z organizatorami...',
  })
}

const teamSchema = z.object({
  name: z.string().min(1, 'Team name is required'),
  size: z.number().min(1).max(5, 'Team size must be between 1 and 5'),
})

const schema = z.object({
  teams: z.array(teamSchema).min(1, 'At least one team is required').max(3, 'You can register up to 3 teams'),
})

type Schema = z.output<typeof schema>

const state = reactive<Schema>({
  teams: [
    { name: '', size: 1 },
  ],
})

async function onSubmit(event: FormSubmitEvent<Schema>) {
  const transformedData = {
    teams: event.data.teams.map(team => [team.name, team.size] as [string, number]),
  }

  toast.add({ title: 'Success', description: 'The form has been submitted.', color: 'success' })
  console.error(event.data)

  await useNuxtApp().$api('/teams/external_invitations/create/{confirmation_code}', {
    path: {
      confirmation_code: confirmationCode,
    },
    method: 'POST',
    body: transformedData,
  })

  await refresh()
}

function addTeam() {
  if (state.teams!.length < 3) {
    state.teams?.push({ name: '', size: 1 })
  }
}

function removeTeam(index: number) {
  if (state.teams!.length > 1) {
    state.teams?.splice(index, 1)
  }
}

function print() {
  if (import.meta.browser) {
    window.print()
  }
}
</script>

<template>
  <div class="max-w-3xl mx-auto p-6 bg-gray-800 rounded-lg shadow-lg">
    <h1 class="text-2xl font-semibold text-center text-white mb-6">
      Zarejestruj drużynę
    </h1>
    <span>Dziękujemy za zainteresowanie przez szkołę <strong class="text-bold">{{ school }}</strong> konkursem Hack4Krak!</span>

    <div v-if="data && data?.length > 0">
      <p class="my-5">
        Gratulacje! Poniżej znajduje się lista unikalnych kodów rejestracyjnych dla każdego uczestnika. Proszę przekazać każdemu uczniowi jego indywidualny kod lub wydrukować i rozdać odpowiednie fragmenty.

        Następnie każdy uczeń powinien założyć konto na stronie hack4krak.pl, kliknąć przycisk dołączyć do drużyny i przepisać lub zeskanować otrzymany kod rejestracyjny.
      </p>
      <UButton icon="mdi:printer" class="print:hidden" @click="print">
        Wydrukuj
      </UButton>
      <div v-for="item in data as any" :key="item.team_name" class="my-5">
        <span class="font-bold">Nazwa drużyny: {{ item.team_name }}</span>
        <div class="flex gap-2 mt-1">
          <span
            v-for="code in item.codes"
            :key="code"
            class="px-2 py-1 bg-blue-100 text-blue-800 rounded text-sm print:text-black"
          >
            {{ code }}
            <Qrcode
              :value="code"
              class="w-16"
            />
          </span>
        </div>
      </div>
    </div>

    <UForm v-else :schema="schema" :state="state" class="space-y-6" @submit="onSubmit">
      <div v-for="(team, index) in state.teams" :key="index" class="space-y-4 p-4 bg-gray-700 rounded-lg shadow-sm">
        <h3 class="text-xl font-semibold text-gray-100">
          Drużyna {{ index + 1 }}
        </h3>

        <UFormField label="Nazwa Drużyny" name="{`teams[${index}].name`}">
          <UInput v-model="team.name" class="w-full p-2 bg-gray-600 text-white border border-gray-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500" />
        </UFormField>

        <UFormField label="Wielkosć Drużyny" name="{`teams[${index}].size`}">
          <UInput v-model="team.size" type="number" min="1" max="5" class="w-full p-2 bg-gray-600 text-white border border-gray-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500" />
        </UFormField>

        <UButton v-if="state.teams.length > 1" type="button" class="w-full bg-red-600 text-white p-2 rounded-lg hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500" @click="removeTeam(index)">
          Usuń drużynę
        </UButton>
      </div>

      <UButton v-if="state.teams.length < 3" type="button" class="w-full bg-blue-600 text-white p-2 rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500" @click="addTeam">
        Dodaj drużynę
      </UButton>

      <UButton type="submit" class="w-full bg-green-600 text-white p-2 rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500">
        Wyślij
      </UButton>
    </UForm>
  </div>
</template>
