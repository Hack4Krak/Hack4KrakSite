<script setup lang="ts">
import { LANDING_PARTNER_OFFERS, PARTNER_MAIL_URL } from '~~/content/landing/community'
import { CONTACT_MAIL, CONTACT_PHONE } from '~~/content/organization'

const { proxy } = useScriptUmamiAnalytics()
</script>

<template>
  <section id="partnerzy" class="scroll-mt-(--ui-header-height) py-24 lg:py-32">
    <UContainer class="grid gap-12 lg:grid-cols-[1fr_1.2fr] lg:gap-20">
      <div class="flex flex-col gap-6">
        <SectionHeading align="left" size="lg" subtitle="Dla partnerów" title="Zbudujmy razem kolejną edycję" />
        <p class="text-base leading-relaxed text-default/80 lg:text-lg">
          Udział w Hack4Krak jest dla uczniów <span class="pixel-underline">bezpłatny</span>. Sale, jedzenie, nagrody i dojazdy uczestników
          opłacają partnerzy. Szukamy firm i instytucji, które chcą, żeby młodzi ludzie uczyli się
          cyberbezpieczeństwa w praktyce.
        </p>

        <div class="mt-2 flex flex-wrap items-center gap-6">
          <ElevatedButton
            :to="PARTNER_MAIL_URL"
            background="var(--ui-primary)"
            class="text-base lg:text-lg"
            @click="proxy.track('partner_cta_click', { location: 'partner_call' })"
          >
            Poproś o ofertę
          </ElevatedButton>
          <div class="flex flex-col text-sm">
            <a :href="`mailto:${CONTACT_MAIL}`" class="font-bold text-default hover:text-primary">{{ CONTACT_MAIL }}</a>
            <a :href="`tel:${CONTACT_PHONE.replaceAll(' ', '')}`" class="text-muted hover:text-primary">{{ CONTACT_PHONE }}</a>
          </div>
        </div>
      </div>

      <div class="flex flex-col gap-5">
        <ColorCard
          v-for="offer in LANDING_PARTNER_OFFERS"
          :key="offer.title"
          :color="offer.color"
          :icon="offer.icon"
          :title="offer.title"
          :description="offer.description"
        />
      </div>
    </UContainer>

    <UContainer class="mt-24 border-t-2 border-surface-muted pt-20 lg:mt-32 lg:pt-24">
      <TrustedBy />
    </UContainer>
  </section>
</template>
