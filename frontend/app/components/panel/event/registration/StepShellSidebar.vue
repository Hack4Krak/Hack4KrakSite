<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const props = defineProps<{
  step: 'agreements' | 'personal' | 'team' | 'confirmation'
}>()

const event = LANDING_CONTENT.event

const STEP_HINTS: Record<typeof props.step, { number: string, title: string, body: string }> = {
  agreements: {
    number: '01',
    title: 'Po co te zgody?',
    body: 'Hack4Krak to wydarzenie stacjonarne — przetwarzamy dane uczestników zgodnie z RODO. Część zgód możesz zaakceptować tutaj. Pozostałe zgody muszą zostać podpisane i dostarczone w dniu wydarzenia.',
  },
  personal: {
    number: '02',
    title: 'Po co te dane?',
    body: 'Imię i nazwisko trafią na identyfikator. Wyżywienie pomaga zaplanować catering. Telefon do opiekuna kontaktujemy tylko w sytuacjach awaryjnych.',
  },
  team: {
    number: '03',
    title: 'Drużyna',
    body: 'Możesz startować solo albo w zespole do 5 osób. Lider zaprasza pozostałych po nicku. Brak drużyny nie blokuje rejestracji — możesz dołączyć później.',
  },
  confirmation: {
    number: '04',
    title: 'Co dalej?',
    body: 'Po wysłaniu otrzymasz mailem QR kod uczestnika — pokażesz go przy wejściu w dniu wydarzenia. Ten sam kod znajdziesz w panelu wydarzenia.',
  },
}

const hint = computed(() => STEP_HINTS[props.step])
</script>

<template>
  <aside class="hidden lg:block self-start sticky top-28">
    <PanelCard body class="space-y-5">
      <div>
        <PanelSectionTitle>
          Hack4Krak 2026
        </PanelSectionTitle>
        <p class="font-pixelify text-2xl text-default mt-2 leading-none">
          {{ event.dateDisplay }}
        </p>
      </div>

      <dl class="space-y-3 text-sm">
        <div>
          <dt>
            <PanelDataLabel>
              Miejsce
            </PanelDataLabel>
          </dt>
          <dd class="font-medium mt-1 leading-snug">
            {{ event.venue.name }}
          </dd>
          <dd class="text-xs text-muted mt-0.5">
            {{ event.venue.address }}
          </dd>
        </div>
        <div>
          <dt>
            <PanelDataLabel>
              Wielkość zespołu
            </PanelDataLabel>
          </dt>
          <dd class="font-medium mt-1">
            1-5 osób
          </dd>
        </div>
      </dl>

      <Transition name="hint" mode="out-in">
        <div :key="step" class="pt-5 border-t border-surface-muted/60 flex gap-3">
          <span class="font-pixelify text-2xl leading-none text-primary tabular-nums shrink-0">
            {{ hint.number }}
          </span>
          <div class="space-y-1.5">
            <p class="text-xs uppercase tracking-wider text-default font-bold">
              {{ hint.title }}
            </p>
            <p class="text-xs text-muted leading-relaxed">
              {{ hint.body }}
            </p>
          </div>
        </div>
      </Transition>
    </PanelCard>
  </aside>
</template>

<style scoped>
.hint-enter-active,
.hint-leave-active {
  transition:
    opacity 160ms ease,
    transform 200ms cubic-bezier(0.22, 1, 0.36, 1);
}
.hint-enter-from {
  opacity: 0;
  transform: translateY(0.5rem);
}
.hint-leave-to {
  opacity: 0;
  transform: translateY(-0.5rem);
}
</style>
