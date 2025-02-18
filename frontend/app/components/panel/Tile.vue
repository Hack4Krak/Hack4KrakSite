<script setup lang="ts">
import Placeholder from '~/components/primitive/Placeholder.vue'
</script>

<template>
  <div class="relative tile bor tile border border-neutral-700 rounded-3xl">
    <div class="overflow-hidden w-full h-full">
      <div class="h-full rounded-3xl overflow-scroll">
        <slot>
          <Placeholder class="w-full h-full" />
        </slot>
      </div>
    </div>
  </div>
</template>
