<script setup lang="ts">
defineProps<{
  id: string
  name: string
  difficultyEstimate: string
  labels: string[]
  isCompleted: boolean
}>()
</script>

<template>
  <div
    class="bg-default p-3 rounded-md border border-border hover:bg-muted cursor-pointer transition-all"
  >
    <div class="flex items-start justify-between gap-2 mb-2">
      <h3 class="font-semibold text-sm flex-1">
        {{ name }}
      </h3>
      <span
        v-if="isCompleted"
        class="px-2 py-0.5 rounded bg-green-100 dark:bg-green-900 text-xs text-green-700 dark:text-green-300 whitespace-nowrap"
        aria-label="Zadanie rozwiązane"
      >
        ✓
      </span>
    </div>
    <div class="flex items-center gap-2 mb-2">
      <span class="text-xs text-muted-foreground">Trudność:</span>
      <span class="px-2 py-0.5 rounded bg-muted text-xs text-muted-foreground">
        {{ difficultyEstimate }}
      </span>
    </div>
    <div class="flex flex-wrap gap-1">
      <span
        v-for="label in labels.slice(0, 3)"
        :key="label"
        class="px-1.5 py-0.5 rounded bg-muted text-xs text-muted-foreground"
      >
        {{ label }}
      </span>
      <span
        v-if="labels.length > 3"
        class="px-1.5 py-0.5 rounded bg-muted text-xs text-muted-foreground"
      >
        +{{ labels.length - 3 }}
      </span>
    </div>
  </div>
</template>
