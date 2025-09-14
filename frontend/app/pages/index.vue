<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

useSeoMeta({
  title: 'Strona Główna',
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
    <UContainer class="w-full mb-2">
      <section class="my-8">
        <SectionInfoBox />
      </section>
      <section class="my-8">
        <SectionInfoTiles />
      </section>
      <Footer class="mt-8 lg:mt-12" />
    </UContainer>
  </main>
</template>
