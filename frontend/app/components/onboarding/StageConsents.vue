<script setup lang="ts">
import { LANDING_SOCIALS } from '~~/content/landing-socials'

const SOCIAL_LINKS = LANDING_SOCIALS.filter(s => ['Discord', 'Instagram', 'LinkedIn'].includes(s.label))

const marketingConsent = defineModel<boolean>('marketingConsent', { required: true })
const collabInterest = defineModel<boolean>('collabInterest', { required: true })

const CONSENT_ITEMS = [
  {
    key: 'marketingConsent' as const,
    title: 'Chcę dostawać informacje o kolejnych edycjach Hack4Krak',
    description: 'Maks. kilka maili rocznie. Wypisać się można w każdej chwili.',
    confirmedText: 'Dzięki — nie ominą Cię żadne następne wydarzenia.',
  },
  {
    key: 'collabInterest' as const,
    title: 'Chcę współtworzyć kolejne edycje Hack4Krak',
    description: 'Pomoc przy zadaniach, organizacji albo mentoringu.',
    confirmedText: 'Super! Niedługo się odezwiemy. \n Wpadnij też na Discorda poznać ekipę.',
  },
]

const models = { marketingConsent, collabInterest }
</script>

<template>
  <section class="space-y-5">
    <div>
      <h2 class="text-lg font-semibold sm:text-xl">
        Ostatnie dwie rzeczy
      </h2>
      <p class="text-sm text-muted">
        Powiedz, na ile chcesz pozostać z nami w kontakcie.
      </p>
    </div>

    <div
      v-for="item in CONSENT_ITEMS"
      :key="item.key"
      class="border-2 p-4 bg-elevated/40 transition-colors"
      :class="models[item.key].value ? 'border-primary/60 bg-primary/5' : 'border-transparent'"
    >
      <label class="flex items-start gap-3 cursor-pointer">
        <USwitch
          :model-value="models[item.key].value"
          class="mt-0.5"
          @update:model-value="(val) => (models[item.key].value = val)"
        />
        <div class="flex-1">
          <p class="text-sm font-medium leading-snug sm:text-base">
            {{ item.title }}
          </p>
          <p class="text-sm text-muted mt-1">
            {{ item.description }}
          </p>
          <Transition name="reveal">
            <div
              v-if="models[item.key].value"
              class="text-sm text-primary mt-2 font-medium"
            >
              {{ item.confirmedText }}
            </div>
          </Transition>
        </div>
      </label>
    </div>

    <div class="pt-2 text-center">
      <p class="text-sm text-muted mb-2">
        Dołącz też do społeczności
      </p>
      <div class="flex justify-center items-center gap-5">
        <NuxtLink
          v-for="social in SOCIAL_LINKS"
          :key="social.label"
          :to="social.to"
          :aria-label="social.label"
          target="_blank"
          class="text-muted hover:text-primary hover:scale-110 transition duration-200"
        >
          <UIcon :name="social.icon" class="w-7 h-7" />
        </NuxtLink>
      </div>
    </div>
  </section>
</template>

<style scoped>
.reveal-enter-active,
.reveal-leave-active {
  transition:
    opacity 250ms ease,
    transform 250ms ease,
    max-height 250ms ease;
  overflow: hidden;
}
.reveal-enter-from,
.reveal-leave-to {
  opacity: 0;
  max-height: 0;
  transform: translateY(-4px);
}
.reveal-enter-to,
.reveal-leave-from {
  opacity: 1;
  max-height: 4rem;
  transform: translateY(0);
}
</style>
