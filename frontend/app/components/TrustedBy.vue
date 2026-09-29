<script setup lang="ts">
import type { LandingPartner } from '~~/content/landing/partners'
import { LANDING_PARTNERS, LANDING_TRUSTED_PARTNERS } from '~~/content/landing/partners'

withDefaults(defineProps<{
  subtitle?: string
  title?: string
}>(), {
  subtitle: 'Partnerzy i patroni',
  title: 'Zaufali nam',
})

const { proxy } = useScriptUmamiAnalytics()

const others = LANDING_PARTNERS.filter(partner => !LANDING_TRUSTED_PARTNERS.includes(partner))

// Wide wordmarks get shorter and square badges taller, so every logo covers a similar area
const VISUAL_AREA = 5200
const MIN_HEIGHT = 26
const MAX_HEIGHT = 62

function logoHeight(partner: LandingPartner, scale = 1) {
  const height = Math.min(MAX_HEIGHT, Math.max(MIN_HEIGHT, Math.sqrt(VISUAL_AREA / (partner.logoAspect ?? 3))))
  return `${Math.round(height * scale)}px`
}
</script>

<template>
  <section class="flex flex-col items-center gap-12 lg:gap-14">
    <SectionHeading :subtitle="subtitle" :title="title" />

    <ul class="grid w-full max-w-6xl grid-cols-2 items-start gap-x-8 gap-y-10 sm:grid-cols-3 lg:grid-cols-6">
      <li v-for="partner in LANDING_TRUSTED_PARTNERS" :key="partner.name">
        <a
          :href="partner.url"
          target="_blank"
          rel="noopener"
          :title="partner.name"
          class="group flex flex-col items-center gap-3"
          @click="proxy.track('partner_open', { partner: partner.name })"
        >
          <span class="flex h-16 items-center">
            <NuxtImg
              :src="partner.logo"
              :alt="partner.logoAlt ?? partner.name"
              height="128"
              class="w-auto max-w-full object-contain transition-transform group-hover:-translate-y-1"
              :style="{ height: logoHeight(partner) }"
            />
          </span>
          <span class="text-center text-[0.65rem] font-bold tracking-[0.12em] text-balance text-muted uppercase">
            {{ partner.tagline }}
          </span>
        </a>
      </li>
    </ul>

    <div class="w-full">
      <p class="mb-4 text-center text-xs font-bold tracking-[0.25em] text-muted uppercase">
        Oraz
      </p>
      <Marquee :items="others" :duration="45">
        <template #default="{ item }">
          <a
            :href="item.url"
            target="_blank"
            rel="noopener"
            :title="`${item.name}: ${item.tagline}`"
            class="flex h-16 items-center px-6 opacity-60 transition-opacity hover:opacity-100"
            @click="proxy.track('partner_open', { partner: item.name })"
          >
            <NuxtImg
              :src="item.logo"
              :alt="item.logoAlt ?? item.name"
              height="96"
              class="w-auto max-w-none"
              :style="{ height: logoHeight(item, 0.7) }"
            />
          </a>
        </template>
      </Marquee>
    </div>
  </section>
</template>
