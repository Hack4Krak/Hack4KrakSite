<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as party from 'party-js'
import * as z from 'zod'

const flagPattern = /^hack4KrakCTF\{.*\}$/
const schema = z.object({
  flag: z.string({ error: 'Wpisz flagę' })
    .regex(flagPattern, { error: 'Flaga musi być w formacie "hack4KrakCTF{...}"' }),
})

type Schema = z.output<typeof schema>

const state = reactive<Partial<Schema>>({
  flag: undefined,
})

const toast = useToast()
const { $auth } = useNuxtApp()

async function onSubmit(event: FormSubmitEvent<Schema>) {
  const response = await $auth('/flag/submit', {
    method: 'POST',
    body: {
      flag: event.data.flag,
    },
  }).catch()

  if ((response as any).error) {
    return
  }

  const target = event.target as HTMLElement | undefined
  if (target) {
    party.confetti(target, {
      count: party.variation.range(300, 700),
      spread: 30,
    })
  }

  toast.add({ title: 'Brawo! To była poprawna flaga', description: getRandomJoke(), color: 'success', duration: 5000 })
  state.flag = undefined
}
</script>

<template>
  <UForm :schema="schema" :state="state" class="space-y-4 flex flex-col" @submit="onSubmit">
    <UFormField name="flag">
      <UInput v-model="state.flag" class="w-full" :ui="{ base: 'h-12' }" placeholder="hack4KrakCTF{...}" />
    </UFormField>

    <ElevatedButton class="w-40" type="submit">
      Wyślij
    </ElevatedButton>
  </UForm>
</template>
