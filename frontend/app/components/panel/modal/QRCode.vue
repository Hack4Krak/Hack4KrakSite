<script setup lang="ts">
const emit = defineEmits<{
  (e: 'codeScanned', code: string): void
}>()

const open = defineModel<boolean>()

function onDetect(detectedCodes: DetectedBarcode[]) {
  emit('codeScanned', detectedCodes[0]!.rawValue)
}
</script>

<template>
  <UModal v-model:open="open" title="Zeskanuj kod QR">
    <template #body>
      <QrcodeStream
        @detect="onDetect"
      />
    </template>
  </UModal>
</template>
