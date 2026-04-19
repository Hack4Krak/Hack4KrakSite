<script setup lang="ts">
import type { AuthResponse } from '#open-fetch'

defineProps<{
  tags: AuthResponse<'account_identification'>['applied_tags']
}>()
</script>

<template>
  <div class="bg-surface-muted border-2 border-surface-muted max-w-xl">
    <div class="px-4 py-3 border-b border-surface-muted">
      <h2 class="font-bold text-lg">
        Przypisane tagi
      </h2>
    </div>
    <div class="space-y-px">
      <div
        v-for="tag in tags"
        :key="tag.id"
        class="px-4 py-3 border-b border-surface-muted last:border-b-0 flex items-start justify-between"
      >
        <div class="flex-1">
          <p class="font-semibold text-sm">
            {{ tag.name }}
          </p>
          <p class="text-xs text-muted mt-1">
            {{ tag.description }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>
