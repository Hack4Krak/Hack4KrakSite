<script setup lang="ts">
import { DISCORD_URL } from '~~/content/landing/socials'
import { CONTACT_MAIL } from '~~/content/organization'

const { proxy } = useScriptUmamiAnalytics()

const WAYS = [
  {
    icon: 'pixelarticons:user-plus',
    color: '#00c950',
    title: 'Dołącz do organizatorów',
    description: 'Piszesz zadania, fotografujesz, ogarniasz logistykę albo po prostu chcesz pomóc? Współorganizuj z nami kolejną edycję.',
    cta: 'Wejdź na Discorda',
    to: DISCORD_URL,
    external: true,
  },
  {
    icon: 'pixelarticons:briefcase',
    color: '#ffdf26',
    title: 'Zostań partnerem wydarzenia',
    description: 'Własne zadanie w konkursie, prelekcja albo stoisko na edycji dla czterech krajów. Zobacz, co zyskuje firma, i poproś o ofertę.',
    cta: 'Oferta dla partnerów',
    to: '/#partnerzy',
    external: false,
  },
  {
    icon: 'pixelarticons:radio-signal',
    color: '#dab2ff',
    title: 'Zostań patronem',
    description: 'Prowadzisz media, serwer Discord, koło naukowe albo organizację dla młodzieży? Pomóż nam dotrzeć do uczniów, a my pokażemy Was na wydarzeniu.',
    cta: 'Napisz do nas',
    to: `mailto:${CONTACT_MAIL}?subject=${encodeURIComponent('Patronat Hack4Krak')}`,
    external: true,
  },
]
</script>

<template>
  <section class="py-24 lg:py-36">
    <UContainer class="flex flex-col gap-12 lg:gap-14">
      <div class="flex flex-col items-center gap-6 text-center">
        <SectionHeading subtitle="Dołącz do nas" title="Następna edycja może być też Twoja" highlight="Twoja" />
        <p class="max-w-2xl text-base leading-relaxed text-default/80 lg:text-lg">
          Każda edycja Hack4Krak była większa od poprzedniej, bo dołączali do nas kolejni ludzie i partnerzy.
          W 2027 zapraszamy uczniów z czterech krajów i potrzebujemy do tego więcej rąk.
        </p>
      </div>

      <div class="grid gap-4 lg:grid-cols-3 lg:gap-6">
        <ColorCard
          v-for="way in WAYS"
          :key="way.title"
          :color="way.color"
          :icon="way.icon"
          :title="way.title"
          :description="way.description"
          :to="way.to"
          :cta="way.cta"
          :external="way.external"
          @click="proxy.track('about_cta_click', { action: way.title })"
        />
      </div>
    </UContainer>
  </section>
</template>
