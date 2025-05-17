<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

useSeoMeta({
  title: 'Strona Główna',
  description: 'Krakowski CTF dla wszystkich uczniów szkół średnich! przetestuj się i zawalcz o ciekawe nagrody biorąc udział w wydarzeniu! Nie zwlekaj i zgłoś swoją drużynę już dziś!',
})
</script>

<template>
  <main>
    <HeroSection>
      <template #title>
        {{ LANDING_CONTENT.title }}
      </template>
      <template #content>
        {{ LANDING_CONTENT.subTitle }}
      </template>
    </HeroSection>
    <UContainer class="w-full mb-24">
      <section class="my-8">
        <SectionInfoBox />
      </section>
      <section class="my-8">
        <SectionInfoTiles />
      </section>
      <section id="section-how-to-participate" class="my-8">
        <LazySectionHowToParticipate hydrate-on-interaction />
      </section>
      <Footer hydrate-on-idle class="mt-8 lg:mt-12" />
    </UContainer>
  </main>
</template>
