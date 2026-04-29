<script setup lang="ts">
import {
  LANDING_PARTICIPATION_STEPS,
} from '~~/content/landing-participation'
</script>

<template>
  <div class="hidden lg:grid lg:grid-cols-4 gap-4 relative">
    <div
      v-for="(step, index) in LANDING_PARTICIPATION_STEPS"
      :key="step.title"
      class="relative flex flex-col items-center gap-4 border-2 bg-default p-6"
      :class="step.highlight ? 'shadow-[0_0_0_2px_color-mix(in_oklch,var(--color-primary)_35%,transparent)]' : ''"
      :style="{ borderColor: step.color }"
    >
      <div class="flex items-center justify-between w-full">
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
        <p class="text-sm text-default/70 leading-relaxed">
          {{ step.description }}
        </p>
      </div>

      <div
        v-if="index < LANDING_PARTICIPATION_STEPS.length - 1"
        class="absolute -right-3 top-1/2 -translate-y-1/2 z-10 flex h-6 w-6 items-center justify-center bg-default"
      >
        <UIcon name="pixelarticons:chevron-right" class="size-5 text-muted" />
      </div>
    </div>
  </div>

  <div class="flex flex-col gap-0 lg:hidden relative">
    <div class="absolute left-5 top-0 bottom-0 w-0.5 bg-surface-muted" aria-hidden="true" />

    <div
      v-for="(step, index) in LANDING_PARTICIPATION_STEPS"
      :key="step.title"
      class="relative flex items-start gap-5 pb-8 last:pb-0"
    >
      <div
        class="relative z-10 flex-shrink-0 w-10 h-10 border-2 flex items-center justify-center
                  font-pixelify text-lg font-bold bg-default"
        :class="step.highlight ? 'bg-primary/10' : ''"
        :style="{ borderColor: step.color, color: step.color }"
      >
        {{ index + 1 }}
      </div>

      <div class="pt-1 flex-1" :class="step.highlight ? 'border-l-2 pl-4 -ml-0.5' : ''" :style="step.highlight ? { borderColor: step.color } : {}">
        <div class="flex items-center gap-2 mb-1">
          <p class="font-bold text-base" :style="{ color: step.color }">
            {{ step.title }}
          </p>
          <UIcon
            v-if="step.highlight"
            name="pixelarticons:check"
            class="size-4"
            :style="{ color: step.color }"
          />
        </div>
        <p class="text-sm text-default/70 leading-relaxed">
          {{ step.description }}
        </p>
      </div>
    </div>
  </div>

  <div class="flex flex-col sm:flex-row items-center justify-center mt-10 lg:mt-14 gap-8">
    <ElevatedButton
      to="/register"
      class="text-base lg:text-lg lg:scale-110"
    >
      Zarejestruj się
    </ElevatedButton>

    <SectionElementsDiscordButton />
  </div>
</template>
