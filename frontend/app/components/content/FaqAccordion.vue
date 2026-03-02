<script setup lang="ts">
const props = defineProps<{
  question: string
}>()

const items = computed(() => [{
  label: props.question,
}])
</script>

<template>
  <UAccordion
    trailing-icon="pixelarticons:chevron-down"
    :ui="{
      label: 'text-lg',
      trailingIcon: 'text-primary size-6',
      trigger: 'cursor-pointer',
      body: 'cursor-text text-md font-light',
    }"
    :items="items"
    :unmount-on-hide="false"
  >
    <template #body>
      <slot />
    </template>
  </UAccordion>
</template>
