<script setup lang="ts">
import type { LandingPartner } from '~~/content/landing-partners'

const props = defineProps<{
  partner: LandingPartner
}>()

const isFeatured = computed(() => props.partner.variant === 'featured')
const isPlaceholder = computed(() => props.partner.variant === 'placeholder')

const isExternalLink = computed(() => {
  try {
    const { protocol } = new URL(props.partner.url)
    return protocol === 'https:' || protocol === 'http:'
  } catch {
    return false
  }
})

const ariaLabel = computed(() =>
  isPlaceholder.value ? 'Zostań partnerem' : `Partner: ${props.partner.name}`,
)

const cardClass = computed(() => {
  if (isFeatured.value)
    return 'relative w-70 lg:w-[320px] border-primary bg-primary/5 px-14 py-12 gap-4 hover:bg-primary/10'
  return isPlaceholder.value
    ? 'w-[240px] border-dashed border-surface-muted px-8 py-8 gap-3 opacity-50 hover:border-primary hover:opacity-100'
    : 'w-[240px] border-surface-muted px-8 py-8 gap-3 hover:border-primary'
})

const mediaClass = computed(() =>
  isFeatured.value
    ? 'h-24 w-full flex items-center justify-center'
    : 'h-20 flex items-center justify-center transition-colors duration-300',
)

const mediaTextClass = computed(() =>
  isPlaceholder.value ? 'text-muted group-hover:text-primary' : 'text-default',
)

const logoClass = computed(() =>
  isFeatured.value
    ? 'max-h-24 max-w-60 w-auto h-auto object-contain transition-all duration-300'
    : 'max-h-20 max-w-[180px] w-auto h-auto object-contain transition-all duration-300',
)
</script>

<template>
  <a
    :href="partner.url"
    :target="isExternalLink ? '_blank' : undefined"
    :rel="isExternalLink ? 'noopener noreferrer' : undefined"
    class="group flex flex-col items-center justify-center border-2 transition-all duration-300 cursor-pointer"
    :class="cardClass"
    :aria-label="ariaLabel"
    :data-partner-variant="partner.variant"
  >
    <span
      v-if="isFeatured"
      class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-primary text-default text-[10px] font-bold tracking-widest uppercase px-4 py-1 font-pixelify whitespace-nowrap"
    >
      {{ partner.tagline }}
    </span>

    <div :class="[mediaClass, mediaTextClass]">
      <NuxtImg
        v-if="partner.logo"
        :src="partner.logo"
        :alt="partner.logoAlt ?? partner.name"
        :class="logoClass"
      />
      <span
        v-else
        class="font-pixelify tracking-widest font-bold text-3xl"
      >
        {{ partner.name }}
      </span>
    </div>

    <span
      v-if="!isFeatured"
      class="text-xs text-center text-muted transition-colors duration-300 group-hover:text-default"
    >
      {{ partner.tagline }}
    </span>
  </a>
</template>
