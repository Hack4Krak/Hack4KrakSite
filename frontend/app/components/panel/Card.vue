<script setup lang="ts">
const props = withDefaults(defineProps<{
  title?: string
  titleClass?: string
  accent?: boolean
  danger?: boolean
  dashed?: boolean
  body?: boolean
}>(), {
  titleClass: 'text-primary',
})

const cardClass = computed(() => [
  'border-2 bg-default shadow-[inset_0_0_0_1px_rgb(250_250_250/0.025)]',
  props.accent ? 'border-primary bg-primary/5' : props.danger ? 'border-error/40 bg-error/5' : 'border-surface-muted',
  props.dashed ? 'border-dashed' : '',
])
</script>

<template>
  <section :class="cardClass">
    <header v-if="title || $slots.header" class="flex items-center justify-between gap-4 px-5 py-3">
      <slot name="header">
        <h2 class="text-sm font-bold uppercase tracking-wider" :class="titleClass">
          {{ title }}
        </h2>
      </slot>
    </header>

    <div v-if="body || title || $slots.header" class="p-5 lg:p-6">
      <slot />
    </div>
    <slot v-else />
  </section>
</template>
