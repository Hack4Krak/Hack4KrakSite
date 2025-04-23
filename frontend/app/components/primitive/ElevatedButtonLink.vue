<script setup lang="ts">
import type { RouteLocationRaw } from 'vue-router'

interface Props {
  to: RouteLocationRaw
  external?: boolean
  target?: '_blank' | '_self' | '_parent' | '_top'
}

withDefaults(defineProps<Props>(), {
  external: false,
  target: '_self',
})
</script>

<template>
  <NuxtLink
    class="elevated-button bg-red block"
    :to="to"
    :target="target"
    :external="external"
  >
    <span
      class="block bg-cream text-dark text-md font-bold
             px-4 py-2 border-2 border-dark shadow-cream
             -translate-x-1 -translate-y-1 duration-300
             hover:-translate-y-1.5 hover:-translate-x-1.5 hover:cursor-pointer
             active:translate-x-0 active:translate-y-0"
    >
      <slot>
        Submit
      </slot>
    </span>
  </NuxtLink>
</template>
