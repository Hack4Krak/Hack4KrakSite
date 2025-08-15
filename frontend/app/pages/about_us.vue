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
