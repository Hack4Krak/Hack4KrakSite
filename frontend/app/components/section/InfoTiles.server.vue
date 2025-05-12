<script setup lang="ts">
import { INFO_TILES_CONTENT } from '~~/content/info-tiles'
</script>

<template>
  <div class="flex flex-col lg:grid grid-cols-3 gap-2 w-full auto-cols-auto">
    <HeroInfoTile
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
          <ElevatedButton class="lg:text-xl lg:scale-110 bg-red-450">
            Zarejestruj swoją drużynę!
          </ElevatedButton>
        </NuxtLink>
      </div>
      <NuxtImg src="/img/character/dragon.png" alt="czerwony smok" class="rendering-pixelated w-32 lg:w-64" />
    </div>
  </div>
</template>
