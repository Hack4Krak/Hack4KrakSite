<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

defineProps<{
  step: 'agreements' | 'personal' | 'team' | 'confirmation'
}>()

const event = LANDING_CONTENT.event

const facts = [
  { label: 'Termin', value: event.dateDisplay },
  { label: 'Miejsce', value: event.venue.name },
  { label: 'Adres', value: event.venue.address },
  { label: 'Max. zespół', value: '5 osób' },
]
</script>

<template>
  <aside class="panel-card hidden lg:block self-start">
    <header class="panel-card-header">
      <p class="text-xs uppercase tracking-wider text-primary font-bold">
        Hack4Krak 2026
      </p>
    </header>

    <ul class="panel-card-body space-y-3 pb-0">
      <li
        v-for="fact in facts"
        :key="fact.label"
        class="panel-subcard"
      >
        <p class="text-xs uppercase tracking-wider text-muted">
          {{ fact.label }}
        </p>
        <p class="font-medium text-sm mt-0.5">
          {{ fact.value }}
        </p>
      </li>
    </ul>

    <div v-if="step === 'agreements'" class="panel-card-body space-y-2">
      <p class="text-xs uppercase tracking-wider text-muted font-bold">
        Dlaczego prosimy o zgody?
      </p>
      <p class="text-xs text-muted leading-relaxed">
        Hack4Krak to fizyczne wydarzenie. Organizator przetwarza dane uczestników zgodnie z RODO.
        Zgody oznaczone <span class="text-success font-bold">online</span> potwierdzasz teraz.
        Zgody <span class="text-primary font-bold">przy wejściu</span> podpisujesz papierowo w dniu wydarzenia.
      </p>
    </div>

    <div v-else-if="step === 'personal'" class="panel-card-body space-y-2">
      <p class="text-xs uppercase tracking-wider text-muted font-bold">
        Po co te dane?
      </p>
      <p class="text-xs text-muted leading-relaxed">
        Imię i nazwisko trafi na identyfikator. Wyżywienie pomaga zaplanować catering.
        Telefon do opiekuna kontaktujemy tylko w sytuacjach awaryjnych.
      </p>
    </div>

    <div v-else-if="step === 'team'" class="panel-card-body space-y-2">
      <p class="text-xs uppercase tracking-wider text-muted font-bold">
        Drużyna
      </p>
      <p class="text-xs text-muted leading-relaxed">
        Możesz startować solo albo w zespole do 5 osób. Lider zaprasza pozostałych po nicku.
        Brak drużyny nie blokuje rejestracji — możesz dołączyć później.
      </p>
    </div>

    <div v-else-if="step === 'confirmation'" class="panel-card-body space-y-2">
      <p class="text-xs uppercase tracking-wider text-muted font-bold">
        Co dalej?
      </p>
      <p class="text-xs text-muted leading-relaxed">
        Po wysłaniu otrzymasz mailem QR kod uczestnika. Pokażesz go przy wejściu w dniu wydarzenia.
        Ten sam kod znajdziesz w panelu wydarzenia.
      </p>
    </div>
  </aside>
</template>
