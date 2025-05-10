<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as z from 'zod'

const props = defineProps<{
  confirmationCode: string
  onSuccess: () => void
}>()

const toast = useToast()

const teamSchema = z.object({
  name: z.string().min(1, 'Proszę podać name drużyny'),
  size: z.number().min(1).max(5),
})

const schema = z.object({
  teams: z.array(teamSchema).min(1).max(3),
})

type Schema = z.output<typeof schema>

const state = reactive<Schema>({
  teams: [
    { name: '', size: 1 },
  ],
})

function addTeam() {
  if (state.teams.length < 3) {
    state.teams.push({ name: '', size: 1 })
  }
}

function removeTeam(index: number) {
  if (state.teams.length > 1) {
    state.teams.splice(index, 1)
  }
}

async function onSubmit(event: FormSubmitEvent<Schema>) {
  const transformedData = {
    teams: event.data.teams.map(t => [t.name, t.size] as [string, number]),
  }

  await useNuxtApp().$api('/teams/external_invitations/create/{confirmation_code}', {
    path: { confirmation_code: props.confirmationCode },
    method: 'POST',
    body: transformedData,
  })

  toast.add({ title: 'Sukces', description: 'Pomyślnie zarejestrowano', color: 'success' })

  props.onSuccess()
}
</script>

<template>
  <UForm :schema="schema" :state="state" class="space-y-6" @submit="onSubmit">
    <p>
      Prosimy o wpisanie nazw drużyn oraz liczby uczestników.
    </p>

    <div v-for="(team, index) in state.teams" :key="index" class="space-y-4 p-4 bg-elevated rounded-lg shadow-sm">
      <div class="flex gap-2">
        <UFormField label="Nazwa" :name="`teams.${index}.name`">
          <UInput v-model="team.name" />
        </UFormField>
        <UFormField label="Ilość uczestników" :name="`teams.${index}.size`">
          <UInput v-model="team.size" type="number" :min="1" :max="5" />
        </UFormField>
        <UButton v-if="state.teams.length > 1" icon="mdi:delete-forever" type="button" class="self-start mt-6" @click="removeTeam(index)" />
      </div>
    </div>

    <div class="flex gap-5">
      <UButton v-if="state.teams.length < 3" color="neutral" @click="addTeam">
        Dodaj drużynę
      </UButton>

      <UButton type="submit" color="primary">
        Wyślij
      </UButton>
    </div>
  </UForm>
</template>
