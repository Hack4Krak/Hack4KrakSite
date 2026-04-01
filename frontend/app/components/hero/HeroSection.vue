<script setup lang="ts">
import type { ComponentExposed } from 'vue-component-type-helpers'
import type HeroContent from './HeroContent.vue'
import LANDING_CONTENT from '~~/content/landing-page.json'

defineSlots<ComponentExposed<typeof HeroContent>['$slots']>()

const { event } = LANDING_CONTENT
</script>

<template>
  <section
    class="lg:h-screen-without-header flex flex-col items-center w-full pt-0 lg:pt-0 lg:gap-0"
  >
    <LazyHeroDesktopHeroBackground hydrate-on-visible class="h-3/4" />
    <LazyHeroMobileLogoSection hydrate-on-visible />

    <!-- Content row: title/cta left, social right -->
    <div
      class="flex-1 flex flex-col lg:flex-row items-center bg-transparent
             gap-y-6 lg:gap-0 place-content-between xl:w-(--ui-container) px-6 pb-10 lg:pb-0"
    >
      <HeroContent>
        <template v-for="(_, name) in $slots" #[name]="slotProps">
          <slot :name="name" v-bind="slotProps ?? {}" />
        </template>

        <!-- Event strip injected as CTA slot — shares the -top-20 offset of HeroContent -->
        <template v-if="event.registrationOpen" #cta>
          <div class="hidden lg:flex items-stretch event-strip">
            <!-- Primary-colored accent bar with pulsing dot -->
            <div class="event-strip-accent flex items-center justify-center px-4 flex-shrink-0">
              <span class="live-dot" aria-hidden="true" />
            </div>

            <!-- Info + button -->
            <div class="flex flex-1 items-center justify-between gap-6 px-5 py-3.5">
              <div>
                <p class="text-[9px] font-black tracking-[0.35em] uppercase text-primary leading-none mb-1.5">
                  Zapisy otwarte
                </p>
                <p class="text-sm font-semibold text-default leading-snug">
                  {{ event.dateDisplay }} &bull; {{ event.venue.city }}
                </p>
              </div>
              <NuxtLink :to="event.registrationUrl" aria-label="Zapisz się na Hack4Krak CTF">
                <ElevatedButton class="text-sm whitespace-nowrap">
                  Zapisz się
                </ElevatedButton>
              </NuxtLink>
            </div>
          </div>
        </template>
      </HeroContent>

      <HeroSocialMediaContainer />
    </div>

    <!-- Mobile CTA -->
    <LazyHeroCallToAction hydrate-on-visible class="lg:hidden mb-10" />
  </section>
</template>

<style scoped>
.event-strip {
  border: 2px solid var(--color-primary);
  max-width: 440px;
  background: var(--color-default);
  box-shadow:
    0 0 28px color-mix(in oklch, var(--color-primary) 22%, transparent),
    0 2px 12px rgba(0, 0, 0, 0.4);
}

.event-strip-accent {
  background: var(--color-primary);
  min-width: 44px;
}

.live-dot {
  display: block;
  width: 9px;
  height: 9px;
  border-radius: 50%;
  background: var(--color-default);
  position: relative;
  animation: pulse-scale 2s ease-in-out infinite;
}

.live-dot::after {
  content: '';
  position: absolute;
  inset: -5px;
  border-radius: 50%;
  border: 2px solid var(--color-default);
  animation: pulse-ring 2s ease-in-out infinite;
  opacity: 0;
}

@keyframes pulse-scale {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(0.6); }
}

@keyframes pulse-ring {
  0% { transform: scale(0.4); opacity: 1; }
  100% { transform: scale(2); opacity: 0; }
}
</style>
