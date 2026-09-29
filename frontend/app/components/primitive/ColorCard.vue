<script setup lang="ts">
// Bordered card in one of the pixel palette colours, in the style of the old "how to participate" steps
defineProps<{
  color: string
  icon: string
  title: string
  description: string
  to?: string
  cta?: string
  external?: boolean
}>()

defineEmits<{
  click: []
}>()

const ULink = resolveComponent('ULink')
</script>

<template>
  <component
    :is="to ? ULink : 'div'"
    :to="to"
    :target="to && external ? '_blank' : undefined"
    class="flex flex-col gap-4 border-2 bg-default p-6 lg:p-7"
    :style="{ borderColor: color }"
    @click="$emit('click')"
  >
    <div class="flex items-center justify-between gap-4">
      <p class="text-xl font-bold lg:text-2xl" :style="{ color }">
        {{ title }}
      </p>
      <UIcon :name="icon" class="size-7 shrink-0" :style="{ color }" />
    </div>
    <p class="text-sm leading-relaxed text-default/72 lg:text-base">
      {{ description }}
    </p>
    <span v-if="cta" class="mt-auto inline-flex items-center gap-2 font-bold" :style="{ color }">
      {{ cta }}
      <UIcon name="pixelarticons:arrow-right" class="size-5" />
    </span>
  </component>
</template>
