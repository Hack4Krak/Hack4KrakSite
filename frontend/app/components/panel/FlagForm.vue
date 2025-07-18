<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as party from 'party-js'

const flagPattern = /^hack4KrakCTF\{.*\}$/
const schema = z.object({
  flag: z.string({ error: 'Wpisz flagę' })
    .regex(flagPattern, { error: 'Flaga musi być w formacie "hack4KrakCTF{...}"' }),
})

type Schema = zInfer<typeof schema>

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

  toast.add({ title: 'Brawo! To była poprawna flaga', description: getRandomJoke(), color: 'success', duration: 12500 })
  state.flag = undefined
}
</script>

<template>
  <UForm :schema="schema" :state="state" class="space-y-3 flex flex-col text-center items-center justify-center" @submit="onSubmit">
    <h3 class="font-bold text-xl">
      Podaj Flagę
    </h3>
    <UFormField name="flag">
      <UInput v-model="state.flag" class="w-80" :ui="{ base: 'h-12 rounded-none' }" placeholder="hack4KrakCTF{...}" />
    </UFormField>

    <ElevatedButton class="w-40 mt-3" type="submit">
      Wyślij
    </ElevatedButton>
  </UForm>
</template>
