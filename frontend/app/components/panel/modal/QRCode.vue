<script setup lang="ts">
const emit = defineEmits<{
  (e: 'codeScanned', code: string): void
}>()

const open = defineModel<boolean>()

function onDetect(detectedCodes: DetectedBarcode[]) {
  emit('codeScanned', detectedCodes[0]!.rawValue)
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
  <UModal v-model:open="open" title="Zeskanuj kod QR">
    <template #body>
      <QrcodeStream
        @detect="onDetect"
        @error="onError"
      />
    </template>
  </UModal>
</template>
