<script setup lang="ts">
const schema = z.object({
  code: z.array(z.string({ error: 'Kod do rejestracji jest wymagany' })).length(6, 'Kod do rejestracji musi mieć 6 znaków'),
})

type Schema = zInfer<typeof schema>

const qrCodeModal = ref(false)
const open = defineModel<boolean>()

async function onSubmit(data: Schema) {
  await useNuxtApp().$auth('/teams/external_invitations/join', {
    method: 'POST',
    body: {
      code: data.code.join(''),
    },
  })

  useToast().add({ title: 'Sukces', description: 'Pomyślnie dołączyłeś team', color: 'success' })
  open.value = false
  await refreshNuxtData()
}

function codeScanned(code: string, state?: Partial<Schema>) {
  if (code.length !== 6) {
    return useToast().add({
      title: `Niepoprawny kod QR`,
      description: 'Spróbuj zeskanować kod ponownie',
      color: 'error',
    })
  }

  if (state) {
    state.code = code.split('')
  }

  qrCodeModal.value = false
}
</script>

<template>
  <AutoFormModal
    v-model:open="open"
    title="Dołącz do zespołu"
    description="Aby wziąć udział w tym wydarzeniu nauczyciel z Twojej szkoły musi zarejestrować drużyne!"
    :schema="schema"
    @submit="onSubmit"
  >
    <template #code="{ field, state }">
      <LazyPanelModalQRCodeScan v-model="qrCodeModal" hydrate-on-idle @code-scanned="code => codeScanned(code, state)" />

      <div class="flex items-center space-x-2">
        <UPinInput
          :model-value="state[field]"
          class="mr-5"
          :length="6"
          @update:model-value="value => state[field] = value"
        />
        <UButton icon="lucide:qr-code" @click="qrCodeModal = true" />
      </div>
    </template>
  </AutoFormModal>
</template>
