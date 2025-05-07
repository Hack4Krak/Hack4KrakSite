<script setup lang="ts">
import { INFO_TILES_CONTENT } from '~~/content/info-tiles'
import LANDING_CONTENT from '~~/content/landing-page.json'
import InfoTile from '~/components/hero/info/InfoTile.vue'

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
    <UContainer class="w-full mb-24">
      <section class="my-8">
        <div class="flex flex-col gap-8 border-green-400 border-2 bg-green-400/10 p-6 text-center justify-center">
          <p class="text-xl lg:text-3xl font-bold text-green-400">
            {{ LANDING_CONTENT.infoBox }}
          </p>
        </div>
      </section>
      <section class="my-8">
        <div class="flex flex-col lg:grid grid-cols-3 gap-2 w-full auto-cols-auto">
          <InfoTile
            v-for="(info, index) in INFO_TILES_CONTENT"
            :key="`tile-${index}`"
            :content="info"
          />
          <div
            class="col-span-2 border-red-450 border-2 flex flex-col px-4 py-12 text-center lg:text-left
                 lg:flex-row justify-center gap-12 items-center"
          >
            <div class="">
              <p class="lg:text-2xl font-bold text-red-450">
                Chcesz wziąć udział?
              </p> <br>
              <NuxtLink to="/panel">
                <ElevatedButton class="lg:text-xl lg:scale-110">
                  Zarejestruj swoją drużynę!
                </ElevatedButton>
              </NuxtLink>
            </div>
            <NuxtImg src="/img/character/dragon.png" class="rendering-pixelated w-32 lg:w-64" />
          </div>
        </div>
      </section>
      <section id="section-how-to-participate" class="my-8">
        <SectionHowToParticipate />
      </section>
      <Footer class="mt-8 lg:mt-12" />
    </UContainer>
  </main>
</template>
