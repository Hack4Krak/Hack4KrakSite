<script setup lang="ts">
import file from 'assets/faq.json?raw'
const content = JSON.parse(file);
const items = ref([
  {
    label: 'Na czym polega \"lokalny CTF\"?',
    content: 'Lokalny CTF to zawody z cyberbezpieczeństwa, które będą się składały z dwóch części: pierwsza to pilotażowy CTF, tylko dla uczniów 31 LO, a druga to główne zawody, otwarte dla wszystkich uczniów szkół średnich w Krakowie. Głównym zadaniem uczestników będzie rozwiązywanie różnych zadań związanych z bezpieczeństwem komputerowym, przełamywaniem zabezpieczeń oraz kryptografią., żeby zdobyć tzw. \"flagę\".'
  },
  {
    label: 'Kto organizuje te zawody w Krakowie?',
    content: 'Zawody organizują uczniowie 31 LO w Krakowie, którzy znają się na cyberbezpieczeństwie i chcą podzielić się tym z innymi.'
  },
  {
    label: 'Kto może wziąć udział w zawodach?',
    content: 'Pierwsza część (testowa) jest tylko dla uczniów 31 LO, ale druga część to zawody otwarte dla wszystkich uczniów szkół średnich w Krakowie i okolic. Można się zapisać indywidualnie lub w drużynie.'
  },
  {
    label: 'Jak mogę się zarejestrować?',
    content: 'Wystarczy zalogować się na stronie, stworzyć drużynę i już!'
  },
  {
    label: 'Gdzie odbędą się zawody?',
    content: 'Zawody będą w 31 LO w Krakowie.'
  },
  {
    label: 'Jak mogę skontaktować się z organizatorami w razie pytań?',
    content: 'Można napisać przez formularz na stronie lub wysłać maila na adres: [adres e-mail].'
  },
  {
    label: 'Czy korzystanie ze sztucznej inteligencji, w tym filmów na YouTube i innych narzędzi AI, jest dozwolone?',
    content: 'Oczywiście, można używać AI, filmów na YouTube i innych źródeł w trakcie zawodów. Możesz korzystać z nich, aby szukać wskazówek, analizować zadania czy korzystać z narzędzi, które pomogą ci rozwiązać problemy.'
  },
  {
    label: 'Czy (miejsce na twoje pytanie).',
    content: 'Nie.'
  }

])
</script>

<template>
    <div class="font-semibold ml-14 mt-8 text-5xl text-yellow-400">FAQ</div>
    <div class="font-light ml-14 mt-3 text-2xl">Czyli najczęśniej zadawane pytania, które nigdy nie padły :O</div>
    <UAccordion class='ml-8 md:ml-24 lg:ml-24 mt-16 w-5/6 md:w-[calc(100%-16rem)]' type='multiple' trailing-icon="i-lucide-arrow-down" :items="items" />
</template>