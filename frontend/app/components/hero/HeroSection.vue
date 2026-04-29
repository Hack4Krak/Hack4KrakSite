<script setup lang="ts">
import type { ComponentExposed } from 'vue-component-type-helpers'
import type HeroContent from './HeroContent.vue'
import LANDING_CONTENT from '~~/content/landing/page'

defineSlots<ComponentExposed<typeof HeroContent>['$slots']>()

const event = LANDING_CONTENT.event

const { data: registrationInformation } = await useApi('/event/registration')

const registrationOpen = computed(() => registrationInformation.value?.is_open ?? false)
</script>

<template>
  <section
    class="lg:h-screen-without-header relative flex flex-col items-center w-full overflow-hidden"
  >
    <LazyHeroDesktopHeroBackground hydrate-on-visible class="absolute inset-0" />
    <div class="absolute inset-x-0 bottom-0 hidden h-[16rem] bg-default lg:block" />

    <LazyHeroMobileLogoSection hydrate-on-visible />

    <div class="hidden lg:block absolute bottom-0 left-0 right-0 pb-14 z-10">
      <UContainer class="flex items-end gap-12 lg:gap-16 xl:gap-20">
        <div class="flex min-w-0 flex-1 flex-col gap-4">
          <HeroContent>
            <template v-for="(_, name) in $slots" #[name]="slotProps">
              <slot :name="name" v-bind="slotProps ?? {}" />
            </template>
          </HeroContent>
          <HeroSocialMediaContainer />
        </div>

        <div v-if="registrationOpen" class="ml-auto flex-shrink-0 translate-y-1">
          <div class="flex items-center gap-5 border-2 border-primary bg-default/95 px-5 py-3">
            <div class="flex-1">
              <p class="font-pixelify text-sm font-bold text-primary leading-none mb-1.5">
                Rejestracja drużyn trwa
              </p>
              <p class="text-sm font-semibold text-default leading-snug whitespace-nowrap">
                {{ event.dateDisplay }} &bull; {{ event.venue.city }} &bull; udział bezpłatny
              </p>
            </div>
            <ElevatedButton
              to="#jak-wziac-udzial"
              class="text-sm whitespace-nowrap flex-shrink-0"
              :ui="{ inner: 'px-4 py-2.5' }"
              aria-label="Dowiedz się jak wziąć udział"
            >
              Zobacz więcej
            </ElevatedButton>
          </div>
        </div>
      </UContainer>
    </div>

    <div class="lg:hidden flex-1 flex flex-col items-center gap-y-6 px-6 pb-6">
      <HeroContent>
        <template v-for="(_, name) in $slots" #[name]="slotProps">
          <slot :name="name" v-bind="slotProps ?? {}" />
        </template>
      </HeroContent>
      <HeroSocialMediaContainer />
    </div>
  </section>
</template>
