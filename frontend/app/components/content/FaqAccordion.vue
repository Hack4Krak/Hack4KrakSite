<script setup lang="ts">
defineProps<{
  question: string
}>()

const open = ref(false)
</script>

<template>
  <div class="border-t border-[var(--ui-border)]">
    <button
      class="flex items-center justify-between w-full cursor-pointer py-3.5 text-lg font-medium text-left"
      @click="open = !open"
    >
      <span>{{ question }}</span>
      <UIcon
        name="mdi:chevron-down"
        class="text-primary size-6 shrink-0 transition-transform duration-200"
        :class="{ 'rotate-180': open }"
      />
    </button>
    <div v-show="open" class="pb-3.5 text-md font-light">
      <slot />
    </div>
  </div>
</template>
