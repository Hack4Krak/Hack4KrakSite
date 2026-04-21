<script setup lang="ts">
import type { EventCardProps } from '~~/content/about-us-timeline'
import { breakpointsTailwind, useBreakpoints } from '@vueuse/core'
import { tv } from 'tailwind-variants'

const props = withDefaults(defineProps<EventCardProps>(), {
  addColorAccent: false,
})

const open = ref(false)
const isHoverMode = useBreakpoints(breakpointsTailwind).greater('md')
const contentMarkdown = await parseMarkdown(props.description)

const card = tv({
  slots: {
    root: 'sm:w-90 w-80 border-2 flex flex-col bg-default cursor-pointer transition-shadow duration-200 hover:shadow-lg',
    imgWrapper: 'h-40 border-b-2',
    contentWrapper: 'border-b-2 flex-1 flex flex-col justify-center p-8',
    button: 'flex-1 flex items-center justify-center',
    content: 'ring-2 p-8',
    userNumber: 'w-30 flex gap-1.5 leading-none items-center justify-center',
    expandedContent: 'border-t-2 p-8 overflow-hidden',
  },
  variants: {
    color: {
      primary: {
        root: 'border-primary',
        imgWrapper: 'border-primary',
        contentWrapper: 'border-primary',
        button: 'bg-primary text-default',
        content: 'ring-primary',
        expandedContent: 'border-primary',
      },
      neutral: {
        root: 'border-muted',
        imgWrapper: 'border-muted',
        contentWrapper: 'border-muted',
        button: 'bg-text-muted text-inverted',
        content: 'ring-text-muted',
        userNumber: 'text-muted',
        expandedContent: 'border-muted',
      },
    },
  },
})

const {
  root,
  imgWrapper,
  contentWrapper,
  button,
  content,
  userNumber,
  expandedContent,
} = card({ color: props.addColorAccent ? 'primary' : 'neutral' })

function toggle() {
  if (!isHoverMode.value) {
    open.value = !open.value
  }
}
</script>

<template>
  <div class="flex flex-col items-center">
    <!-- Desktop: popover on hover -->
    <UPopover
      v-if="isHoverMode"
      v-model:open="open"
      mode="click"
      :content="{ sideOffset: 16 }"
      :ui="{
        content: content(),
      }"
    >
      <div
        :class="root()"
        @mouseenter="open = true"
        @mouseleave="open = false"
      >
        <div :class="imgWrapper()">
          <NuxtImg
            width="360"
            height="160"
            :modifiers="props.imgCrop ? {
              crop: props.imgCrop,
            } : undefined"
            :src="img" alt="card-background-event-image" class="w-full h-full object-cover"
          />
        </div>
        <div :class="contentWrapper()">
          <p class="text-muted">
            {{ subtitle }}
          </p>
          <p class="text-lg font-semibold">
            {{ title }}
          </p>
        </div>
        <div class="h-8 flex">
          <div :class="userNumber()">
            <p>{{ participants }}</p>
            <UIcon name="pixelarticons:user" class="size-sm" />
          </div>
          <div :class="button()">
            <Transition
              enter-from-class="opacity-0 -translate-y-1" leave-to-class="opacity-0 translate-y-1"
              enter-active-class="transition-all duration-200 ease-in-out"
              leave-active-class="transition-all duration-200 ease-in-out"
              mode="out-in"
            >
              <UIcon v-if="!open" key="down" name="pixelarticons:chevron-down" class="size-lg" />
              <UIcon v-else key="up" name="pixelarticons:chevron-up" class="size-lg" />
            </Transition>
          </div>
        </div>
      </div>
      <template #content>
        <div class="md:max-w-90 max-w-75">
          <span>
            <MDCRenderer
              :body="contentMarkdown.body" :data="contentMarkdown.data"
              class="text-default prose-p:my-0 prose-p:[&:not(:last-child)]:mb-4"
            />
          </span>
        </div>
      </template>
    </UPopover>

    <!-- Mobile: inline expand -->
    <div v-else :class="root()" @click="toggle">
      <div :class="imgWrapper()">
        <NuxtImg
          width="360"
          height="160"
          :modifiers="props.imgCrop ? {
            crop: props.imgCrop,
          } : undefined"
          :src="img" alt="card-background-event-image" class="w-full h-full object-cover"
        />
      </div>
      <div :class="contentWrapper()">
        <p class="text-muted">
          {{ subtitle }}
        </p>
        <p class="text-lg font-semibold">
          {{ title }}
        </p>
      </div>
      <div class="h-8 flex">
        <div :class="userNumber()">
          <p>{{ participants }}</p>
          <UIcon name="pixelarticons:user" class="size-sm" />
        </div>
        <div :class="button()">
          <Transition
            enter-from-class="opacity-0 -translate-y-1" leave-to-class="opacity-0 translate-y-1"
            enter-active-class="transition-all duration-200 ease-in-out"
            leave-active-class="transition-all duration-200 ease-in-out"
            mode="out-in"
          >
            <UIcon v-if="!open" key="down" name="pixelarticons:chevron-down" class="size-lg" />
            <UIcon v-else key="up" name="pixelarticons:chevron-up" class="size-lg" />
          </Transition>
        </div>
      </div>
      <Transition
        enter-from-class="max-h-0 opacity-0" enter-to-class="max-h-80 opacity-100"
        leave-from-class="max-h-80 opacity-100" leave-to-class="max-h-0 opacity-0"
        enter-active-class="transition-all duration-300 ease-out"
        leave-active-class="transition-all duration-200 ease-in"
      >
        <div v-if="open" :class="expandedContent()">
          <MDCRenderer
            :body="contentMarkdown.body" :data="contentMarkdown.data"
            class="text-default prose-p:my-0 prose-p:[&:not(:last-child)]:mb-4"
          />
        </div>
      </Transition>
    </div>
  </div>
</template>
