<script setup lang="ts">
import {
  LANDING_FEATURED_PARTNERS,
  LANDING_SUPPORTING_PARTNERS,
  PARTNERS_CONTACT_MAIL,
} from '~~/content/landing/partners'
</script>

<template>
  <div class="flex flex-wrap justify-center items-stretch gap-8 lg:gap-12 mb-8">
    <SectionElementsPartnerCard
      v-for="partner in LANDING_FEATURED_PARTNERS"
      :key="partner.name"
      :partner="partner"
    />
  </div>

  <div class="flex flex-wrap justify-center items-stretch gap-6 lg:gap-8">
    <SectionElementsPartnerCard
      v-for="partner in LANDING_SUPPORTING_PARTNERS"
      :key="partner.name"
      :partner="partner"
    />
  </div>

  <p class="text-center text-xs text-muted mt-10">
    Zainteresowany partnerstwem? Napisz do nas:
    <a :href="`mailto:${PARTNERS_CONTACT_MAIL}`" class="link">{{ PARTNERS_CONTACT_MAIL }}</a>
  </p>
</template>
