<script setup lang="ts">
import { tv } from 'tailwind-variants'

const props = withDefaults(defineProps<{
  id?: string
  pretitle?: string
  title?: string
  sectionClass?: string
  compact?: boolean
  separator?: boolean
}>(), {
  compact: false,
  separator: true,
})

const section = tv({
  slots: {
    root: 'my-8 lg:my-16',
    content: 'w-full py-12 lg:py-20',
    header: 'mx-auto mb-10 max-w-3xl text-center lg:mb-14',
    separator: 'my-8 lg:my-16',
  },
  variants: {
    compact: {
      true: {
        root: 'my-8 lg:my-12',
        content: 'py-0',
      },
    },
  },
})

const { root, content, header, separator } = section({
  compact: props.compact,
})
</script>

<template>
  <div>
    <div :class="root()">
      <section :id="props.id" :class="[content(), props.sectionClass]">
        <div v-if="props.pretitle || props.title || $slots.header" :class="header()">
          <slot name="header">
            <p v-if="props.pretitle" class="text-xs font-bold tracking-[0.25em] uppercase text-muted mb-3">
              {{ props.pretitle }}
            </p>
            <h2 v-if="props.title" class="font-pixelify text-3xl lg:text-5xl text-default">
              {{ props.title }}
            </h2>
          </slot>
        </div>

        <slot />
      </section>
    </div>

    <USeparator v-if="props.separator" :class="separator()" />
  </div>
</template>
