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
    <!-- 1. Event countdown + CTA -->
    <section class="my-8 lg:my-12">
      <SectionEventBanner />
    </section>

    <!-- 2. How to participate -->
    <section class="my-8 lg:my-16">
      <SectionHowItWorks />
    </section>

    <USeparator class="my-8 lg:my-16" />

    <!-- 3. Location + schedule -->
    <section class="my-8 lg:my-16">
      <SectionEventDetails />
    </section>

    <USeparator class="my-8 lg:my-16" />

    <!-- 4. Sponsors -->
    <section class="my-8 lg:my-16">
      <SectionSponsors />
    </section>

    <!-- 5. Past editions stats -->
    <section class="my-8 lg:my-16">
      <SectionStats />
    </section>

    <!-- 6. What is CTF / who can join — replaces FAQ teaser at the bottom -->
    <section class="my-8 lg:my-16">
      <SectionInfoTiles />
    </section>

    <Footer class="mt-8 lg:mt-12" />
  </UContainer>
</template>
