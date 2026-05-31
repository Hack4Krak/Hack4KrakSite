<script setup lang="ts">
defineProps<{
  label: string
  value: string
  danger?: boolean
}>()
</script>

<template>
  <div :class="danger ? 'border border-error/30 p-5 flex items-start justify-between gap-4' : 'bg-default p-5 flex items-center justify-between gap-4'">
    <div>
      <p class="font-medium" :class="[danger && 'text-error']">
        {{ label }}
      </p>
      <p class="text-sm text-muted mt-1">
        {{ value }}
      </p>
    </div>
    <slot />
  </div>
</template>
