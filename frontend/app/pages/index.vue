<script setup lang="ts">
useSeoMeta({
  title: 'Strona Główna',
})

const info_data_md = [
  {
    front: 'Na czym będzie polegać moje <br>**zadanie** podczas wydarzenia??',
    back: 'Twoim zadaniem będzie **rozwiązywanie zadań** z różnych dziedzin cyberbezpieczeństwa. Za każde poprawnie rozwiązane zadanie Twoja drużyna otrzyma punkty, które zadecydują o waszym końcowym wyniku.',
    color: 'green',
    image: '/img/knight.png',
  },
  {
    front: 'Czym jest <br>**Hack4Krak**?',
    back: 'Hack4Krak to organizacja zajmująca się tworzeniem i organizacją wydarzeń typu CTF Jeopardy z zakresu cyberbezpieczeństwa, skierowanych do uczniów Krakowskich szkół publicznych.',
    color: 'yellow',
    image: '/img/princess.png',
  },
  {
    front: '**Kto** może <br>wziąć udział?',
    back: 'W wydarzeniu mogą wziąć udział **uczniowie szkół podstawowych**. W szczególności kierujemy je do **uczniów klas 7 i 8**, którzy chcą spróbować swoich sił w świecie cyberbezpieczeństwa.',
    color: 'gold',
    image: '/img/witch_tower.png',
  },
  {
    front: 'Jakie **nagrody** można zdobyć podczas wydarzenia?',
    back: 'Na najlepsze drużyny czekają **atrakcyjne nagrody**, które będą miłym wyróżnieniem za wysiłek, zaangażowanie i umiejętności pokazane w trakcie rywalizacji.',
    color: 'purple',
    image: '/img/king.png',
  },
]
</script>

<template>
  <main>
    <!-- Hero Section -->
    <HeroSection />

    <!-- Sections -->
    <UContainer class="w-full h-screen">
      <section class="info-section flex flex-col lg:grid grid-cols-3 gap-2 w-full auto-cols-auto">
        <InfoTile
          v-for="(info, index) in info_data_md"
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
