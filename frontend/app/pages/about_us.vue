<script setup lang="ts">
import { aboutUsTimeline } from '~~/content/about-us-timeline'
</script>

<template>
  <div class="flex flex-col md:gap-20 gap-14 pt-12">
    <section
      id="hero"
      class="lg:h-[320px] md:h-[180px] relative flex flex-col justify-center items-center w-full overflow-hidden
             sm:text-center sm:items-center sm:px-0 text-left"
    >
      <UContainer class="flex flex-col justify-center items-center">
        <h1 class="heading-h3 font-semibold text-lg">
          Hack4Krak
        </h1>
        <p class="max-w-[420px]">
          to inicjatywa młodych liderów cyberbezpieczeństwa, których połączyła pasja do technologii, wyzwań i ciągłego
          rozwoju. Tworzymy wydarzenia, które uczą, inspirują i integrują młodzież zainteresowaną światem IT i
          bezpieczeństwa cyfrowego.
        </p>
      </UContainer>
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
    <section id="mission">
      <UContainer class="flex lg:flex-row flex-col-reverse mx-auto gap-12">
        <div class="flex-1 flex flex-col gap-4 sm:px-0">
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
            Wierzymy, że edukacja w zakresie bezpieczeństwa cyfrowego powinna być dostępna, motywująca i prowadzona
            przez
            tych, którzy najlepiej rozumieją swoje pokolenie.
          </p>
        </div>
        <div class="flex-1 justify-center items-center hidden sm:flex">
          <NuxtImg class="w-full px-6 sm:px-0" src="/img/flag_n_clouds.webp" />
        </div>
      </UContainer>
    </section>
    <section id="mini-gallery">
      <UContainer class="grid lg:grid-cols-4 lg:grid-rows-[auto, auto] grid-cols-3 gap-4">
        <NuxtImg class="gallery-small-image" src="/img/about-us/Gallery3.webp" />
        <NuxtImg class="gallery-small-image" src="/img/about-us/Gallery2.webp" />
        <NuxtImg class="gallery-small-image" src="/img/about-us/Gallery1.webp" />
        <div class="lg:row-span-2 lg:col-span-1 col-span-3 flex flex-col gap-2">
          <h3 class="heading-h3 text-lg font-semibold">
            Inicjatywa stworzona
            przez uczniów – dla uczniów.
          </h3>
          <p>
            Jesteśmy grupą młodych entuzjastów cyberbezpieczeństwa z Krakowa, których połączyła pasja do technologii,
            wyzwań i ciągłego rozwoju.
          </p>
          <p>
            Działamy jako niezależna organizacja, której celem jest promowanie wiedzy z zakresu cyberbezpieczeństwa
            wśród młodzieży poprzez organizację wydarzeń typu CTF (Capture The Flag) – konkursów, w których uczestnicy
            rozwiązują zadania z obszaru bezpieczeństwa cyfrowego.
          </p>
        </div>
        <NuxtImg class="col-span-3 h-full object-cover border-2 border-content-secondary" src="/img/about-us/GalleryMain.webp" />
      </UContainer>
    </section>
    <Footer />
  </div>
</template>

<style scoped>
@reference "../assets/css/main.css";

.gallery-small-image {
  @apply hidden md:block w-full h-full px-6 sm:px-0 object-cover border-2 border-content-secondary;
}

.stroked-text-full-screen {
  -webkit-text-stroke-color: transparent;
  -webkit-text-fill-color: var(--ui-bg);
  user-select: none;
  background-color: --alpha(var(--color-accent-primary) / 20%);
  background-clip: text;
  -webkit-text-stroke-width: 0.02em;
}
</style>
