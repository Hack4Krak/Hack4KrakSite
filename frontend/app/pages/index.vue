<script setup lang="ts">
useSeoMeta({
  title: 'Strona Główna',
})

const info_data_md = [
  {
    front: 'Jakie jest moje <br>**zadanie**?',
    back: 'Twoim zadaniem jest stworzenie innowacyjnego projektu, który odpowiada na jedno z wyzwań zaproponowanych podczas Hack4Krak. Może to być aplikacja, prototyp, kampania społeczna – liczy się pomysłowość, zaangażowanie i współpraca z zespołem!',
    color: 'green',
    image: '/img/knight.png',
  },
  {
    front: 'Czym jest <br>**Hack4Krak**?',
    back: 'Hack4Krak to hackathon – intensywny maraton kreatywności, podczas którego młodzi ludzie tworzą rozwiązania dla realnych problemów Krakowa. To okazja do nauki, networkingu i dobrej zabawy!',
    color: 'yellow',
    image: '/img/princess.png',
  },
  {
    front: '**Kto** może <br>wziąść udział?',
    back: 'Każdy uczeń szkoły średniej z Krakowa i okolic! Niezależnie od tego, czy programujesz, projektujesz, kręcisz filmy, czy po prostu masz głowę pełną pomysłów – miejsce znajdzie się dla Ciebie!',
    color: 'gold',
    image: '/img/witch_tower.png',
  },
  {
    front: 'Jakie są <br>**nagrody**?',
    back: 'Na zwycięzców czekają atrakcyjne nagrody – od sprzętu elektronicznego, przez vouchery i wejściówki, aż po możliwość dalszego rozwijania swojego projektu z partnerami Hack4Krak!',
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
