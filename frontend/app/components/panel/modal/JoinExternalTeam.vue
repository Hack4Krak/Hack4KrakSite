<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'
import * as z from 'zod'

const schema = z.object({
  code: z.string({ error: 'Nazwa drużyny jest wymagana' }).length(6, 'Nazwa drużyny musi mieć min 6 znaków'),
})

type Schema = z.output<typeof schema>

const state = reactive<Partial<Schema>>({
  code: undefined,
})

const toast = useToast()
const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

const { $auth } = useNuxtApp()

async function onSubmit(event: FormSubmitEvent<Schema>) {
  const response = await $auth('/teams/external_invitations/join', {
    method: 'POST',
    body: {
      code: event.data.code,
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

  state.code = code
}

const qrCodeModal = ref(false)
</script>

<template>
  <UModal v-model:open="open" title="Dołącz do zespołu" description="Aby wziąć udział w tym wydarzeniu nauczyciel z Twojej szkoły musi zarejestrować drużyne!" :ui="{ footer: 'justify-end' }">
    <template #body>
      <LazyPanelModalQRCode v-model="qrCodeModal" hydrate-on-visible @code-scanned="codeScanned" />

      <UForm ref="form" :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
        <UFormField label="Kod rejestracji" name="code">
          <div class="flex items-center space-x-2">
            <UPinInput class="mr-5" :length="6" @update:model-value="(value) => state.code = value.join('')" />
            <UButton icon="mdi:qrcode-scan" @click="qrCodeModal = true" />
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
