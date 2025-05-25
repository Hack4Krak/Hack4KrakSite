<script setup lang="ts">
import LANDING_PAGE from '~~/content/landing-page.json'
</script>

<template>
  <section class="flex flex-col text-center border-amber-300 border-2 p-6 space-y-4">
    <div>
      <h2 class="text-3xl font-bold mb-2">
        Sponsorzy
      </h2>
      <p>Dziękujemy! Bez ich wsparcia organizacja tego wydarzenia nie byłaby możliwa.</p>
    </div>
    <div class="flex flex-wrap justify-center items-center gap-5">
      <NuxtImg
        v-for="sponsor in LANDING_PAGE.sponsors"
        :key="`sponsor-${sponsor.logo}`"
        :src="sponsor.logo"
        :alt="sponsor.name"
        :title="sponsor.name"
        class="w-50"
      />
    </div>
  </section>
</template>
