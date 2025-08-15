<script setup lang="ts">
import { aboutUsTimeline } from '~~/content/about-us-timeline'
</script>

<template>
  <div class="flex flex-col gap-18 pt-12">
    <section
      id="hero"
      class="lg:h-[320px] md:h-[180px] relative flex flex-col justify-center w-full overflow-hidden
             sm:text-center sm:items-center sm:px-0 px-6 text-left"
    >
      <h1 class="heading-h3 font-semibold text-lg">
        Hack4Krak
      </h1>
      <p class="max-w-[420px]">
        to inicjatywa młodych liderów cyberbezpieczeństwa, których połączyła pasja do technologii, wyzwań i ciągłego
        rozwoju. Tworzymy wydarzenia, które uczą, inspirują i integrują młodzież zainteresowaną światem IT i
        bezpieczeństwa cyfrowego.
      </p>
      <p
        class="lg:text-[320px] md:text-[220px] text-[180px] font-semibold absolute -z-10 text-primary
               stroked-text-full-screen w-screen md:w-auto text-wrap"
      >
        HACK4KRAK
      </p>
    </section>
    <section
      id="timeline"
      class="relative flex justify-center items-center"
    >
      <div class="flex md:flex-row flex-wrap flex-col gap-8 items-center justify-center">
        <EventCard v-for="card in aboutUsTimeline" v-bind="card" :key="card.title" />
      </div>
      <NuxtImg
        class="absolute h-[300px] left-0 md:top-auto top-[200px] -z-20" src="/img/blockAccentVector.svg"
        alt="block accent vector"
        height="300px"
      />
      <NuxtImg
        class="absolute h-[300px] right-0 md:top-auto top-[600px] -z-20 rotate-180" src="/img/blockAccentVector.svg"
        alt="block accent vector"
        height="300px"
      />
    </section>
    <section id="mission" class="flex xl:flex-row flex-col-reverse mx-auto gap-12">
      <div class="sm:w-160 flex-1 flex flex-col gap-4 px-6 sm:px-0">
        <h3 class="heading-h3 text-lg font-semibold">
          Nasza misja
        </h3>
        <p>
          Naszą misją jest inspirowanie i wspieranie młodych ludzi w rozwijaniu pasji do cyberbezpieczeństwa. Tworzymy
          przestrzeń opartą na:
        </p>
        <div class="flex flex-wrap gap-4">
          <Label>zaangażowaniu</Label>
          <Label>praktycznej nauce</Label>
          <Label>dzieleniu się wiedzą</Label>
          <Label>rozwoju kompetencji przyszłości</Label>
        </div>
        <p>
          Wierzymy, że edukacja w zakresie bezpieczeństwa cyfrowego powinna być dostępna, motywująca i prowadzona przez
          tych, którzy najlepiej rozumieją swoje pokolenie.
        </p>
      </div>
      <div class="flex-1 justify-center items-center hidden sm:flex">
        <NuxtImg class="w-full px-6 sm:px-0" src="/img/flag_n_clouds.webp" />
      </div>
    </section>
    <Footer />
  </div>
</template>

<style scoped>
.stroked-text-full-screen {
  -webkit-text-stroke-color: transparent;
  -webkit-text-fill-color: var(--ui-bg);
  user-select: none;
  background-color: --alpha(var(--color-accent-primary) / 20%);
  background-clip: text;
  -webkit-text-stroke-width: 0.02em;
}
</style>
