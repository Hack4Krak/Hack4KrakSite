<script setup lang="ts">
import { NuxtLink } from '#components'

defineOptions({ inheritAttrs: false })

const props = withDefaults(defineProps<{
  background?: string
  variant?: 'dark' | 'light'
  to?: string
  target?: string
  ui?: {
    inner?: string
    label?: string
  }
}>(), {
  variant: 'dark',
})

const attrs = useAttrs()

const buttonType = computed(() => (attrs.type as string | undefined) ?? 'button')
const forwardedAttrs = computed(() => {
  const { type: _, ...rest } = attrs
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
      class="block px-4 py-2 shadow-cream -translate-x-1 -translate-y-1 border-2 duration-300 hover:-translate-y-1.5 hover:-translate-x-1.5 hover:cursor-pointer active:translate-x-0 active:translate-y-0"
      :class="[variant === 'light' ? 'border-dark bg-inverted text-inverted' : 'font-pixelify bg-default border-white', props.ui?.inner]"
    >
      <span class="inline-flex items-center gap-3">
        <slot name="leading" />
        <span class="inline-flex items-center leading-none" :class="props.ui?.label">
          <slot />
        </span>
        <slot name="trailing" />
      </span>
    </span>
  </component>
</template>
