<script setup lang="ts">
defineProps<{
  src: string
}>()

const open = defineModel<boolean>('open', { required: true })
</script>

<template>
  <UModal v-model:open="open" :ui="{ content: 'max-w-7xl bg-transparent shadow-none ring-0' }">
    <template #content>
      <div class="flex items-center justify-center" @click="open = false">
        <NuxtImg
          :src="src"
          class="max-h-[90vh] max-w-full object-contain rounded cursor-pointer"
        />
      </div>
    </template>
  </UModal>
</template>
