<script setup lang="ts">
import { INFO_TILES_CONTENT } from '~~/content/info-tiles'
import InfoTile from '~/components/hero/info/InfoTile.vue'

useSeoMeta({
  title: 'Strona Główna',
})
</script>

<template>
  <main>
    <HeroSection>
      <template #title>
        Hack4Krak
      </template>
      <template #content>
        Pierwszy małopolski CTF kierowany <br>
        do uczniów szkół średnich!
      </template>
    </HeroSection>
    <UContainer class="w-full h-screen">
      <DevOnly>
        <section class="my-8">
          <div class="flex flex-col border-green-400 border-2 bg-green-400/10 p-6 text-center justify-center">
            <p class="text-xl lg:text-3xl font-bold text-green-400">
              Zapisy na zawody dla szkół podstawowych już otwarte!
            </p>
          </div>
        </section>
      </DevOnly>
      <section class="info-section flex flex-col lg:grid grid-cols-3 gap-2 w-full auto-cols-auto">
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
      </section>
    </UContainer>
  </main>
</template>
