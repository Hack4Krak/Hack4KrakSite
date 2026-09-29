<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing/page'

const { event } = LANDING_CONTENT
</script>

<template>
  <section id="nastepna-edycja" class="scroll-mt-(--ui-header-height) overflow-x-clip border-y-2 border-surface-muted py-24 lg:py-32">
    <UContainer class="grid gap-14 lg:grid-cols-[1fr_1.15fr] lg:gap-16">
      <div class="flex flex-col gap-6 lg:self-center">
        <SectionHeading align="left" size="lg" subtitle="Następna edycja" title="Jeden CTF, cztery kraje" />
        <p class="text-base leading-relaxed text-default/80 lg:text-lg">
          W 2027 roku podnosimy poprzeczkę jeszcze wyżej. 20–21 marca chcemy zorganizować w Krakowie
          stacjonarny CTF dla około {{ event.expectedParticipants }} uczniów szkół średnich z całej Grupy
          Wyszehradzkiej: Polski, Czech, Słowacji i Węgier.
        </p>

        <LandingV4Flags />

        <ElevatedButton to="#partnerzy" background="var(--ui-primary)" class="mt-2 self-start text-base">
          Pomóż nam to zrobić
        </ElevatedButton>
      </div>

      <LandingV4Map class="lg:-mt-16 lg:-mr-16 lg:translate-x-8" />
    </UContainer>
  </section>
</template>
