<script setup lang="ts">
import type { SchemaParticipantTag } from '#open-fetch-schemas/api'

defineProps<{
  tags: SchemaParticipantTag[]
  selectedTagId?: string
}>()
</script>

<template>
  <div class="bg-surface-muted border-2 border-surface-muted max-w-xl grow">
    <div class="bg-default p-5 border-b border-surface-muted">
      <h2 class="font-bold text-lg">
        Przypisane atrybuty
      </h2>
    </div>
    <div
      v-if="tags.length > 0"
      class="space-y-px"
    >
      <div
        v-for="tag in tags"
        :key="tag.id"
        class="bg-default p-5 flex items-start justify-between gap-4"
        :class="tag.id === selectedTagId ? 'bg-primary/10' : ''"
      >
        <div class="flex-1">
          <p
            class="font-semibold"
            :class="tag.id === selectedTagId ? 'text-primary' : ''"
          >
            {{ tag.name }}
          </p>
          <p class="text-sm text-muted mt-1">
            {{ tag.description }}
          </p>
        </div>
      </div>
    </div>
    <p
      v-else
      class="bg-default p-5 text-sm text-muted"
    >
      Brak przypisanych tagów
    </p>
  </div>
</template>
