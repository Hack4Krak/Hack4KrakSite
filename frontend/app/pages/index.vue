<script setup lang="ts">
import { INFO_TILES_CONTENT } from '~/content/info-tiles'

useSeoMeta({
  title: 'Strona Główna',
})
</script>

<template>
  <main>
    <!-- Hero Section -->
    <HeroSection />

    <!-- Sections -->
    <UContainer class="w-full h-screen">
      <section class="my-8">
        <div class="flex flex-col border-green-400 border-2 bg-green-400/10 p-6 text-center justify-center">
          <p class="text-xl lg:text-3xl font-bold text-green-400">
            Zapisy na zawody dla szkół podstawowych już otwarte!
          </p>
        </div>
      </section>
      <section class="info-section flex flex-col lg:grid grid-cols-3 gap-2 w-full auto-cols-auto">
        <InfoTile
          v-for="info in INFO_TILES_CONTENT"
          :key="info.id"
          :content="info"
        />
        <div
          class="col-span-2 border-primary border-2 flex flex-col px-4 py-12 text-center lg:text-left
                    lg:flex-row justify-center gap-12 items-center"
        >
          <div class="">
            <p class="lg:text-2xl font-bold text-primary">
              Zainteresowany?
            </p> <br>
            <ElevatedButtonLink to="/panel" class="lg:text-xl lg:scale-110">
              Zarejestruj swoją drużynę!
            </ElevatedButtonLink>
          </div>
          <NuxtImg src="/img/dragon.png" class="rendering-pixelated w-32 lg:w-64" />
        </div>
      </section>
    </UContainer>
  </main>
</template>
