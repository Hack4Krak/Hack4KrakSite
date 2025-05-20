<script setup lang="ts">
const props = defineProps<{
  colorHex: string
}>()
</script>

<template>
  <div class="elevated-button bg-white">
    <span
      :style="{ backgroundColor: props.colorHex }"
      class="block -translate-x-[0.1px] -translate-y-[1px]"
    >
      <span
        class="block text-md font-bold px-4 py-2 shadow-cream -translate-x-1 -translate-y-1
        bg-default text-white border-1
        duration-300"
      >
        <slot />
      </span>
    </span>
  </div>
</template>
