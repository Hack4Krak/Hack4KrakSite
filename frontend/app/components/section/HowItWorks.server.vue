<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

const { event } = LANDING_CONTENT

const steps = [
  {
    title: 'Zarejestruj się',
    description: 'Załóż konto na hack4krak.pl i uzupełnij profil. Rejestracja jest darmowa i zajmuje mniej niż minutę.',
    color: '#05df72',
    icon: 'pixelarticons:user',
  },
  {
    title: 'Stwórz drużynę',
    description: 'Zbierz ekipę do 5 osób. Możesz też dołączyć do istniejącej drużyny kodem zaproszenia.',
    color: '#dab2ff',
    icon: 'pixelarticons:users',
  },
  {
    title: `Przyjedź do Krakowa`,
    description: `${event.dateDisplay} stawiamy się na AGH. Przywieź swój sprzęt i gotowość na wyzwanie — reszta czeka na miejscu.`,
    color: '#ffdf26',
    icon: 'pixelarticons:map-pin',
    highlight: true,
  },
  {
    title: 'Rozwiązuj i wygrywaj',
    description: 'Atakuj zadania z kryptografii, web, forensics i OSINT. Zbieraj punkty, wspinaj się w rankingu, zgarnij nagrody.',
    color: '#d08700',
    icon: 'pixelarticons:flag',
  },
]
</script>

<template>
  <section class="w-full py-12 lg:py-20">
    <div class="text-center mb-10 lg:mb-14">
      <p class="text-xs font-bold tracking-[0.25em] uppercase text-muted mb-3">
        Krok po kroku
      </p>
      <h2 class="font-pixelify text-3xl lg:text-5xl text-default">
        Jak wziąć udział?
      </h2>
    </div>

    <!-- Desktop: horizontal rail -->
    <div class="hidden lg:grid lg:grid-cols-4 gap-4 relative">
      <div
        v-for="(step, index) in steps"
        :key="index"
        class="relative flex flex-col items-center gap-4 border-2 p-6 transition-colors"
        :class="step.highlight ? 'bg-primary/5' : ''"
        :style="{ borderColor: step.color }"
      >
        <!-- Step number + icon -->
        <div
          class="flex items-center justify-between w-full"
        >
          <span
            class="font-pixelify text-4xl font-bold opacity-20"
            :style="{ color: step.color }"
          >{{ index + 1 }}</span>
          <UIcon
            :name="step.icon"
            class="size-6"
            :style="{ color: step.color }"
          />
        </div>

        <div class="w-full">
          <p class="font-bold text-base lg:text-lg mb-2" :style="{ color: step.color }">
            {{ step.title }}
          </p>
          <p class="text-sm text-muted leading-relaxed">
            {{ step.description }}
          </p>
        </div>

        <!-- Arrow connector (except last) -->
        <div
          v-if="index < steps.length - 1"
          class="absolute -right-3 top-1/2 -translate-y-1/2 z-10
                 w-6 h-6 bg-default flex items-center justify-center"
        >
          <UIcon name="pixelarticons:chevron-right" class="size-5 text-muted" />
        </div>
      </div>
    </div>

    <!-- Mobile: compact vertical list -->
    <div class="flex flex-col gap-0 lg:hidden relative">
      <!-- Vertical line -->
      <div class="absolute left-5 top-0 bottom-0 w-0.5 bg-surface-muted" aria-hidden="true" />

      <div
        v-for="(step, index) in steps"
        :key="index"
        class="relative flex items-start gap-5 pb-8 last:pb-0"
      >
        <!-- Numbered circle -->
        <div
          class="relative z-10 flex-shrink-0 w-10 h-10 border-2 flex items-center justify-center
                 font-pixelify text-lg font-bold bg-default"
          :class="step.highlight ? 'bg-primary/10' : ''"
          :style="{ borderColor: step.color, color: step.color }"
        >
          {{ index + 1 }}
        </div>

        <!-- Content -->
        <div class="pt-1 flex-1" :class="step.highlight ? 'border-l-2 pl-4 -ml-0.5' : ''" :style="step.highlight ? { borderColor: step.color } : {}">
          <div class="flex items-center gap-2 mb-1">
            <p class="font-bold text-base" :style="{ color: step.color }">
              {{ step.title }}
            </p>
            <UIcon
              v-if="step.highlight"
              name="pixelarticons:star"
              class="size-4"
              :style="{ color: step.color }"
            />
          </div>
          <p class="text-sm text-muted leading-relaxed">
            {{ step.description }}
          </p>
        </div>
      </div>
    </div>

    <div class="flex justify-center mt-10 lg:mt-14">
      <NuxtLink :to="LANDING_CONTENT.event.registrationUrl">
        <ElevatedButton class="text-base lg:text-lg lg:scale-110">
          Zarejestruj się
        </ElevatedButton>
      </NuxtLink>
    </div>
  </section>
</template>
