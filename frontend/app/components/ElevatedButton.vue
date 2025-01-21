<script setup lang="ts">
defineProps<{
  message: string
}>()
</script>

<template>
  <button
    class="p-2 rounded-lg duration-300 font-pixelify cursor-pointer
          drop-shadow-[0px_5px_#987117] text-xl self-center px-4
          bg-[var(--ui-primary)] hover:brightness-105"
  >
    <span>{{ message }}</span>
  </button>
</template>
