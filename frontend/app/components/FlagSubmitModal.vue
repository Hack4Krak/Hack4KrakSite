<script setup lang="ts">
const isOpen = defineModel<boolean>({ default: false })
</script>

<template>
  <UModal v-model:open="isOpen" title="Złóż flagę" description="Wpisz flagę w formacie hack4KrakCTF{...}" :ui="{ body: 'p-3 sm:p-4' }">
    <template #body>
      <PanelTileFlagForm :show-heading="false" @success="isOpen = false" />
    </template>
  </UModal>
</template>
