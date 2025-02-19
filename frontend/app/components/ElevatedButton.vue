<script setup lang="ts">
defineProps<{
  message: string
}>()
</script>

<template>
  <button
    class="py-2 px-4 rounded-lg duration-300 cursor-pointer drop-shadow-[0px_5px_var(--color-primary-darker)]
    bg-(--ui-primary) self-center text hover:scale-102 decoration-0"
  >
    <span class="font-pixelify text-xl text-(--ui-bg) font-semibold">{{ message }}</span>
  </button>
</template>
