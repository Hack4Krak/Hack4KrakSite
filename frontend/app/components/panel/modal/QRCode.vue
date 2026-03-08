<script setup lang="ts">
const emit = defineEmits<{
  (e: 'codeScanned', code: string): void
}>()

const open = defineModel<boolean>()

const schema = z.object({
  code: z.string().meta({ title: 'Kod QR' }),
})

function onDetect(detectedCodes: DetectedBarcode[]) {
  emit('codeScanned', detectedCodes[0]!.rawValue)
  open.value = false
}

function onError(error: Error) {
  console.error(error)
  open.value = false

  useToast().add({
    title: `Nie udało się zeskanować kodu QR`,
    description: error.message ?? 'Nieznany błąd',
    color: 'error',
  })
}
</script>

<template>
  <AutoFormModal
    v-model:open="open"
    title="Zeskanuj kod QR"
    :schema="schema"
    :config="{ submit: false }"
    submit-label="Zamknij"
    close-label="Anuluj"
    @submit="open = false"
  >
    <template #code>
      <QrcodeStream
        @detect="onDetect"
        @error="onError"
      />
    </template>
    <template #footer="{ close }">
      <UButton label="Zamknij" color="neutral" variant="outline" @click="close" />
    </template>
  </AutoFormModal>
</template>
