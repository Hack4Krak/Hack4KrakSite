<script setup lang="ts">
defineProps<{
  verificationId: string | undefined
}>()
const open = defineModel<boolean>()
</script>

<template>
  <UModal v-model:open="open">
    <template #content>
      <UCard>
        <template #header>
          <div class="flex justify-between items-center">
            <h3 class="text-lg font-semibold">
              Twój kod weryfikacyjny
            </h3>
            <UButton
              icon="i-heroicons-x-mark"
              color="neutral"
              variant="ghost"
              size="sm"
              @click="open = false"
            />
          </div>
        </template>
        <div v-if="verificationId" class="flex flex-col items-center gap-4">
          <Qrcode :value="verificationId" class="w-48 h-48" />
          <p class="text-sm text-gray-400 text-center break-all">
            {{ verificationId }}
          </p>
          <p class="text-xs text-gray-500 text-center">
            Pokaż ten kod organizatorom, aby potwierdzić swoją tożsamość
          </p>
        </div>
        <div v-else class="text-center text-gray-400">
          Brak kodu weryfikacyjnego
        </div>
      </UCard>
    </template>
  </UModal>
</template>
