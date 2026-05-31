<script setup lang="ts">
defineProps<{
  kicker?: string
  title: string
  description?: string
  backTo?: string
  backLabel?: string
}>()
</script>

<template>
  <header class="border-2 border-surface-muted bg-default p-5 shadow-[inset_0_0_0_1px_rgb(250_250_250/0.025)] lg:p-7">
    <NuxtLink
      v-if="backTo"
      :to="backTo"
      class="mb-4 flex items-center gap-1.5 text-xs uppercase tracking-wider text-muted transition-colors hover:text-default"
    >
      <UIcon name="pixelarticons:arrow-left" class="size-4" />
      {{ backLabel ?? 'Wróć' }}
    </NuxtLink>

    <p v-if="kicker" class="mb-2 text-xs font-bold uppercase tracking-wider text-primary">
      {{ kicker }}
    </p>
    <h1 class="font-pixelify text-4xl leading-none lg:text-5xl">
      {{ title }}
    </h1>
    <p v-if="description" class="mt-3 max-w-2xl text-sm leading-relaxed text-muted">
      {{ description }}
    </p>
    <slot />
  </header>
</template>
