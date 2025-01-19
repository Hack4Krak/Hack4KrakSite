<script setup lang="ts">
defineProps<{
  message: string
}>()
</script>

<template>
  <button
    class="p-2 rounded-lg duration-300 font-pixelify font-bold
          drop-shadow-[0_5px_#1A51AC] hover:drop-shadow-[0_5px_#537DC2]
          bg-yellow-500 hover:brightness-105 text-xl text-blue-800 self-center cursor-pointer"
  >
    <span>{{ message }}</span>
  </button>
</template>
