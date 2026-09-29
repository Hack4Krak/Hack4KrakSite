<script setup lang="ts">
import { LANDING_COMMUNITY_PHOTOS, LANDING_STATS } from '~~/content/landing/community'
import { FOUNDATION } from '~~/content/organization'

const rows = computed(() => {
  const half = Math.ceil(LANDING_COMMUNITY_PHOTOS.length / 2)
  return [LANDING_COMMUNITY_PHOTOS.slice(0, half), LANDING_COMMUNITY_PHOTOS.slice(half)]
})
</script>

<template>
  <section id="spolecznosc" class="scroll-mt-(--ui-header-height) py-24 lg:py-32">
    <UContainer class="mb-24 lg:mb-32">
      <ul class="grid grid-cols-2 gap-x-6 gap-y-12 lg:grid-cols-4">
        <li
          v-for="stat in LANDING_STATS"
          :key="stat.label"
          class="flex items-end justify-center gap-4"
        >
          <img :src="stat.image" alt="" aria-hidden="true" class="h-16 w-auto shrink-0 rendering-pixelated lg:h-24">
          <div>
            <p class="font-pixelify text-4xl leading-none lg:text-6xl" :style="{ color: stat.color }">
              <CountUp :value="stat.value" />
            </p>
            <p class="mt-2 text-sm text-muted lg:text-base">
              {{ stat.label }}
            </p>
          </div>
        </li>
      </ul>
    </UContainer>

    <UContainer class="grid gap-10 lg:grid-cols-2 lg:items-end lg:gap-20">
      <SectionHeading align="left" size="lg" subtitle="Kim jesteśmy" title="Od uczniów, dla uczniów" highlight="dla uczniów" />

      <div class="flex flex-col gap-5 text-base leading-relaxed text-default/80 lg:text-lg">
        <p>
          Pierwszą edycję Hack4Krak zorganizowaliśmy w 2025 roku w jednej szkole, XXXI LO w Krakowie.
          Od tamtej pory sami, jako grupa młodych ludzi, tworzymy zadania, zdobywamy partnerów, budujemy
          platformę i organizujemy całe wydarzenie. Robimy to, żeby dzielić się
          <span class="pixel-underline">pasją do cyberbezpieczeństwa</span> z kolejnymi rocznikami.
        </p>
        <p class="text-sm text-muted lg:text-base">
          Oficjalnym organizatorem jest
          <ULink :to="FOUNDATION.url" target="_blank" class="font-bold text-primary underline-offset-4 hover:underline">
            {{ FOUNDATION.name }}
          </ULink>, którą założyliśmy, żeby przygotowywać wydarzenia na większą skalę.
        </p>
      </div>
    </UContainer>

    <div class="mt-16 flex flex-col lg:mt-20">
      <Marquee
        v-for="(row, index) in rows"
        :key="index"
        :items="row"
        :duration="60 + index * 12"
        :reverse="index === 1"
      >
        <template #default="{ item }">
          <PixelPhoto :src="item.src" :alt="item.alt" :width="500" class="w-60 sm:w-72 lg:w-80" />
        </template>
      </Marquee>
    </div>

    <UContainer class="mt-14">
      <ElevatedButton to="/about_us" variant="light" background="var(--ui-primary)" class="text-base">
        Poznaj naszą historię
      </ElevatedButton>
    </UContainer>
  </section>
</template>
