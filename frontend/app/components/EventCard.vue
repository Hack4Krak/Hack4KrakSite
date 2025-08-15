<script setup lang="ts">
import type { EventCardProps } from '~~/content/about-us-timeline'
import { tv } from 'tailwind-variants'

const props = withDefaults(defineProps<EventCardProps>(), {
  color: false,
})

const open = ref(false)
const { width } = useWindowSize()
const mode = computed(() => (width.value >= 1024 ? 'hover' : 'click'))

const cardBase = tv({
  slots: {
    root: 'sm:w-90 w-80 border-2 flex flex-col bg-surface-primary',
    imgWrapper: 'h-40 border-b-2',
    contentWrapper: 'border-b-2 flex-1 flex flex-col justify-center p-8',
    button: 'flex-1 flex items-center justify-center',
    content: 'ring-2 p-8',
    userNumber: 'w-30 flex gap-1.5 leading-none items-center justify-center',
  },
})

const colorCard = tv({
  extend: cardBase,
  slots: {
    root: 'border-primary',
    imgWrapper: 'border-primary',
    contentWrapper: 'border-primary',
    button: 'bg-primary text-content-primary',
    content: 'ring-accent-primary',
  },
})

const neutralCard = tv({
  extend: cardBase,
  slots: {
    root: 'border-content-secondary',
    imgWrapper: 'border-content-secondary',
    contentWrapper: 'border-content-secondary',
    button: 'bg-content-secondary text-surface-primary',
    content: 'ring-content-secondary',
    userNumber: 'text-content-secondary',
  },
})

const { root, imgWrapper, contentWrapper, button, content, userNumber } = props.color ? colorCard() : neutralCard()
</script>

<template>
  <UPopover
    v-model:open="open"
    :mode="mode"
    :content="{ sideOffset: 16 }"
    :ui="{
      content: content(),
    }"
  >
    <div
      :class="root()"
    >
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
        <div :class="userNumber()">
          <p>{{ participants }}</p>
          <SizedIcon icon="pixelarticons:user" format="small" />
        </div>
        <div :class="button()">
          <transition name="icon-fade" mode="out-in">
            <SizedIcon v-if="!open" key="down" icon="pixelarticons:chevron-down" format="large" />
            <SizedIcon v-else key="up" icon="pixelarticons:chevron-up" format="large" />
          </transition>
        </div>
      </div>
    </div>
    <template #content>
      <div class="md:max-w-90 max-w-75">
        <span>
          <LazyMarkdownContent :text="description" class="text-content-primary" />
        </span>
      </div>
    </template>
  </UPopover>
</template>

<style scoped>
.icon-fade-enter-active,
.icon-fade-leave-active {
  transition: all 0.2s ease;
}

.icon-fade-enter-from {
  opacity: 0;
  transform: translateY(-4px);
}

.icon-fade-leave-to {
  opacity: 0;
  transform: translateY(4px);
}
</style>
