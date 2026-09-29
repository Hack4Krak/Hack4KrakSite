<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

useSeoMeta({
  title: 'Strona Główna',
})

const { proxy } = useScriptUmamiAnalytics()
</script>

<template>
  <HeroSection>
    <template #title>
      {{ LANDING_CONTENT.title }}
    </template>
    <template #content>
      {{ LANDING_CONTENT.subTitle }}
    </template>
    <template #cta>
      <div class="flex flex-wrap items-center justify-center gap-6 lg:justify-start">
        <ElevatedButton
          to="#partnerzy"
          background="var(--ui-primary)"
          class="text-base lg:text-lg"
          @click="proxy.track('partner_cta_click', { location: 'hero' })"
        >
          Zostań partnerem
        </ElevatedButton>
        <ElevatedButton to="#spolecznosc" variant="light" background="var(--ui-primary)" class="text-base lg:text-lg">
          Poznaj nas
        </ElevatedButton>
      </div>
    </template>
  </HeroSection>

  <LandingCommunity />

  <LandingNextEdition />

  <LandingPartnerCall />

  <LandingFinale />

  <!-- The skyline above already closes the page, so the footer sits right under it -->
  <Footer :separator="false" />
</template>
