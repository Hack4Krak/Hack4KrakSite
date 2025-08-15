<script setup lang="ts">
import type { EventCardProps } from '~~/content/about-us-timeline'
import { tv } from 'tailwind-variants'

const props = withDefaults(defineProps<EventCardProps>(), {
  color: false,
})

const cardBase = tv({
  slots: {
    root: 'w-90 border-2 flex flex-col',
    imgWrapper: 'h-40 border-b-2',
    contentWrapper: 'border-b-2 flex-1 flex flex-col justify-center p-8',
    button: 'flex-1 flex items-center justify-center',
  },
})

const colorCard = tv({
  extend: cardBase,
  slots: {
    root: 'border-primary',
    imgWrapper: 'border-primary',
    contentWrapper: 'border-primary',
    button: 'bg-primary text-content-primary',
  },
})

const neutralCard = tv({
  extend: cardBase,
  slots: {
    root: 'border-content-secondary',
    imgWrapper: 'border-content-secondary',
    contentWrapper: 'border-content-secondary',
    button: 'bg-content-secondary text-surface-primary',
  },
})

const { root, imgWrapper, contentWrapper, button } = props.color ? colorCard() : neutralCard()
</script>

<template>
  <div :class="root()">
    <div :class="imgWrapper()">
      <NuxtImg :src="img" alt="event-img" class="rendering-pixelated w-full h-full object-cover" />
    </div>
    <div :class="contentWrapper()">
      <p class="secondary-text">
        {{ subtitle }}
      </p>
      <p class="heading-h3">
        {{ title }}
      </p>
    </div>
    <div class="h-8 flex">
      <div class="w-30 flex items-center justify-center">
        {{ participants }}
      </div>
      <div :class="button()">
        <SizedIcon icon="pixelarticons:chevron-down" format="large" />
      </div>
    </div>
  </div>
</template>
