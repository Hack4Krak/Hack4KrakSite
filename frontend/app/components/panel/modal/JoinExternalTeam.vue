<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as z from 'zod'

const schema = z.object({
  code: z.array(z.string({ error: 'Kod do rejestracji jest wymagany' })).length(6, 'Kod do rejestracji musi mieć 6 znaków'),
})

type Schema = z.output<typeof schema>

const state = reactive<Partial<Schema>>({
  code: undefined,
})

const qrCodeModal = ref(false)

const toast = useToast()
const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

const { $auth } = useNuxtApp()

async function onSubmit(event: FormSubmitEvent<Schema>) {
  const response = await $auth('/teams/external_invitations/join', {
    method: 'POST',
    body: {
      code: event.data.code.join(''),
    },
  })

  if ((response as any).error) {
    return
  }

  toast.add({ title: 'Sukces', description: 'Pomyślnie dołączyłeś team', color: 'success' })
  await refreshNuxtData()
}

function codeScanned(code: string) {
  if (code.length !== 6) {
    return toast.add({
      title: `Niepoprawny kod QR`,
      description: 'Spróbuj zeskanować kod ponownie',
      color: 'error',
    })
  }

  state.code = code.split('')
  qrCodeModal.value = false
}
</script>

<template>
  <UModal v-model:open="open" title="Dołącz do zespołu" description="Aby wziąć udział w tym wydarzeniu nauczyciel z Twojej szkoły musi zarejestrować drużyne!" :ui="{ footer: 'justify-end' }">
    <template #body>
      <LazyPanelModalQRCode v-model="qrCodeModal" hydrate-on-visible @code-scanned="codeScanned" />

      <UForm ref="form" :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
        <UFormField label="Kod rejestracji" name="code">
          <div class="flex items-center space-x-2">
            <UPinInput v-model="state.code" class="mr-5" :length="6" />
            <UButton icon="lucide:qr-code" @click="qrCodeModal = true" />
          </div>
        </UFormField>
      </UForm>
    </template>

    <template #footer>
      <UButton label="Zamknij" color="neutral" variant="outline" @click="open = false" />
      <UButton label="Dołącz" @click="formRef?.submit()" />
    </template>
  </UModal>
</template>
