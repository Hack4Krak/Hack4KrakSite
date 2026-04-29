<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

useSeoMeta({
  title: 'Strona Główna',
})
</script>

<template>
  <HeroSection>
    <template #title>
      {{ LANDING_CONTENT.title }}
    </template>
    <template #content>
      {{ LANDING_CONTENT.subTitle }}
    </template>
  </HeroSection>

  <UContainer class="w-full mb-2">
    <LandingSection compact>
      <SectionEventBanner />
    </LandingSection>

    <LandingSection
      id="jak-wziac-udzial"
      pretitle="Krok po kroku"
      title="Jak wziąć udział?"
      section-class="scroll-mt-[15vh]"
    >
      <SectionHowToParticipate />
    </LandingSection>

    <LandingSection pretitle="Gdzie i kiedy" title="Lokalizacja i harmonogram">
      <SectionEventDetails />
    </LandingSection>

    <LandingSection
      pretitle="Edycja 2026"
      title="Co na Was czeka?"
    >
      <SectionHighlights />
    </LandingSection>

    <LandingSection pretitle="Partnerzy i Sponsorzy" title="Wspierają nas" :separator="false">
      <SectionPartners />
    </LandingSection>

    <Footer class="mt-8 lg:mt-12" />
  </UContainer>
</template>
