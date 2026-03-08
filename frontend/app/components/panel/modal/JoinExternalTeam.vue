<script setup lang="ts">
const schema = z.object({
  code: z.string({ error: 'Kod do rejestracji jest wymagany' }).length(6, 'Kod do rejestracji musi mieć 6 znaków').meta({ title: 'Kod rejestracji' }),
})

type Schema = zInfer<typeof schema>

const qrCodeModal = ref(false)
const open = defineModel<boolean>()

async function onSubmit(data: Schema) {
  await useNuxtApp().$auth('/teams/external_invitations/join', {
    method: 'POST',
    body: {
      code: data.code,
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
    state.code = code
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
    submit-label="Dołącz"
    close-label="Zamknij"
    @submit="onSubmit"
  >
    <template #code="{ field, state }">
      <LazyPanelModalQRCode v-model="qrCodeModal" hydrate-on-idle @code-scanned="code => codeScanned(code, state)" />

      <div class="flex items-center space-x-2">
        <UPinInput
          :model-value="state[field]?.split('') ?? []"
          class="mr-5"
          :length="6"
          @update:model-value="value => state[field] = value.join('')"
        />
        <UButton icon="lucide:qr-code" @click="qrCodeModal = true" />
      </div>
    </template>
  </AutoFormModal>
</template>
