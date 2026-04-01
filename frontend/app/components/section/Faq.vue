<script setup lang="ts">
import LANDING_CONTENT from '~~/content/landing-page.json'

const previewFaq = LANDING_CONTENT.faq.slice(0, 4)
</script>

<template>
  <section class="w-full py-12 lg:py-20">
    <!-- Section header -->
    <div class="flex flex-col lg:flex-row lg:items-end lg:justify-between gap-4 mb-10 lg:mb-14">
      <div>
        <p class="text-xs font-bold tracking-[0.25em] uppercase text-muted mb-3">
          Masz pytania?
        </p>
        <h2 class="font-pixelify text-3xl lg:text-5xl text-default">
          FAQ
        </h2>
      </div>
      <NuxtLink
        to="/docs/faq"
        class="flex items-center gap-2 text-sm text-muted hover:text-primary transition-colors duration-200 group w-fit"
      >
        <span>Pełna lista pytań</span>
        <UIcon name="pixelarticons:chevron-right" class="size-4 group-hover:translate-x-0.5 transition-transform" />
      </NuxtLink>
    </div>

    <!-- FAQ items: full-width stacked list with left number index -->
    <div class="flex flex-col divide-y divide-surface-muted border-t border-surface-muted">
      <div
        v-for="(item, index) in previewFaq"
        :key="index"
        class="group"
      >
        <FaqAccordion :question="item.question">
          <p class="text-muted text-sm pb-4 leading-relaxed">
            {{ item.answer }}
          </p>
        </FaqAccordion>
      </div>
    </div>

    <!-- CTA -->
    <div class="flex justify-center mt-10">
      <NuxtLink to="/docs/faq">
        <ElevatedButton class="text-sm">
          Zobacz wszystkie pytania
        </ElevatedButton>
      </NuxtLink>
    </div>
  </section>
</template>
