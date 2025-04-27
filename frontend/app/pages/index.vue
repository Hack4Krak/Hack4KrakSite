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
      <section class="info-section flex flex-col lg:grid grid-cols-3 gap-2 w-full auto-cols-auto">
        <InfoTile
          v-for="(info, index) in INFO_TILES_CONTENT"
          :key="index"
          :content="info"
        />
        <div
          class="col-span-2 border-red border-2 flex flex-col px-4 py-12 text-center lg:text-left
                    lg:flex-row justify-center gap-12 items-center"
        >
          <div class="">
            <p class="lg:text-2xl font-bold text-red">
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
