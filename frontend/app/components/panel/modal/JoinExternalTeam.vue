<script setup lang="ts">
import type { FormSubmitEvent } from '@nuxt/ui'

const schema = z.object({
  code: z.array(z.string({ error: 'Kod do rejestracji jest wymagany' })).length(6, 'Kod do rejestracji musi mieć 6 znaków'),
})

type Schema = zInfer<typeof schema>

const state = reactive<Partial<Schema>>({})
const qrCodeModal = ref(false)
const open = defineModel<boolean>()
const formRef = useTemplateRef('form')

async function onSubmit(event: FormSubmitEvent<Schema>) {
  await useNuxtApp().$auth('/teams/external_invitations/join', {
    method: 'POST',
    body: {
      code: event.data.code.join(''),
    },
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie dołączyłeś team', color: 'success' })
  await refreshNuxtData()
}

function codeScanned(code: string) {
  if (code.length !== 6) {
    return useToast().add({
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
