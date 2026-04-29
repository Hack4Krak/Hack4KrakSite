<script setup lang="ts">
import type { StyleValue } from 'vue'
import { NuxtLink } from '#components'
import { tv } from 'tailwind-variants'

defineOptions({ inheritAttrs: false })

const props = withDefaults(defineProps<{
  background?: string
  variant?: 'dark' | 'light'
  to?: string
  target?: string
  innerStyle?: StyleValue
  ui?: {
    inner?: string
    content?: string
    label?: string
    description?: string
  }
}>(), {
  variant: 'dark',
})

const attrs = useAttrs()

const innerClass = tv({
  base: 'block px-4 py-2 shadow-cream -translate-x-1 -translate-y-1 bg-default border-2 duration-300 hover:-translate-y-1.5 hover:-translate-x-1.5 hover:cursor-pointer active:translate-x-0 active:translate-y-0',
  variants: {
    variant: {
      light: 'border-dark bg-inverted text-inverted',
      dark: 'font-pixelify bg-default border-white',
    },
  },
})

const buttonType = computed(() => (attrs.type as string | undefined) ?? 'button')
const forwardedAttrs = computed(() => {
  const { type: _type, ...rest } = attrs
  return rest
})
</script>

<template>
  <component
    :is="props.to ? NuxtLink : 'button'"
    v-bind="forwardedAttrs"
    :to="props.to"
    :target="props.to ? props.target : undefined"
    :type="props.to ? undefined : buttonType"
    class="inline-block bg-primary-500 text-white text-md font-bold"
    :style="{ backgroundColor: props.background }"
  >
    <span
      :class="[innerClass({ variant: props.variant }), props.ui?.inner]"
      :style="props.innerStyle"
    >
      <span class="inline-flex items-center gap-3" :class="props.ui?.content">
        <slot name="leading" />
        <span v-if="$slots.description" class="flex flex-col items-start text-left" :class="props.ui?.label">
          <span class="leading-none">
            <slot />
          </span>
          <span class="mt-1 text-[0.68rem] tracking-[0.12em] uppercase leading-none text-muted" :class="props.ui?.description">
            <slot name="description" />
          </span>
        </span>
        <span v-else class="inline-flex items-center leading-none" :class="props.ui?.label">
          <slot />
        </span>
        <slot name="trailing" />
      </span>
    </span>
  </component>
</template>
