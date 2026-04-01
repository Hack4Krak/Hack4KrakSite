<script setup lang="ts">
defineSlots<{
  title: () => VNode | VNode[]
  content: () => VNode | VNode[]
  cta?: () => VNode | VNode[]
}>()
</script>

<template>
  <div class="relative lg:-top-20 text-center lg:text-left">
    <h1 class="font-bold pr-4 stroked-text-1 text-5xl sm:text-6xl lg:text-9xl leading-tight">
      <slot name="title" />
    </h1>
    <p class="hero-description text-base sm:text-lg lg:text-2xl whitespace-pre-line mt-3 lg:mt-0 text-muted lg:text-default">
      <slot name="content" />
    </p>
    <div v-if="$slots.cta" class="mt-6 lg:mt-8">
      <slot name="cta" />
    </div>
  </div>
</template>
