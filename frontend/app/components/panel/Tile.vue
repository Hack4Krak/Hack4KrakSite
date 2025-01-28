<script setup lang="ts">
import Placeholder from '~/components/primitive/Placeholder.vue'
</script>

<template>
  <div class="tile border border-neutral-700 rounded-3xl overflow-hidden">
    <div class="p-5 h-full rounded-3xl overflow-scroll">
      <slot>
        <Placeholder class="w-full h-full" />
      </slot>
    </div>
  </div>
</template>
