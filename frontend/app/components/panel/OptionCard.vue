<script setup lang="ts">
defineProps<{
  selected?: boolean
  title: string
  description?: string
}>()

defineEmits<{ select: [] }>()
</script>

<template>
  <button
    type="button"
    class="border-2 px-4 py-3 text-left transition-colors cursor-pointer"
    :class="selected ? 'border-primary bg-primary/5' : 'border-surface-muted hover:border-primary/40'"
    @click="$emit('select')"
  >
    <span class="block text-sm font-bold" :class="selected ? 'text-primary' : 'text-default'">
      {{ title }}
    </span>
    <span v-if="description" class="mt-1 block text-xs text-muted">
      {{ description }}
    </span>
    <slot />
  </button>
</template>
