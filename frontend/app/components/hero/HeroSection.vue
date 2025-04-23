<script setup lang="ts">
import CallToAction from '~/components/hero/CallToAction.vue'
import MobileLogoSection from '~/components/hero/MobileLogoSection.vue'
import SocialMediaContainer from '~/components/hero/SocialMediaContainer.vue'
</script>

<template>
  <section
    class="hero-section lg:h-[calc(100vh-var(--ui-header-height))] flex flex-col items-center
            w-full mb-12 gap-y-12 lg:gap-0"
  >
    <!-- Desktop Background Image -->
    <HeroBackground class="h-3/4" />

    <!-- Mobile Logo -->
    <MobileLogoSection />

    <!-- Hero Content -->
    <div
      class="hero-content flex-1 flex flex-col lg:flex-row items-center bg-transparent
               gap-y-12 lg:gap-0 place-content-between xl:w-(--ui-container)"
    >
      <!-- Title and Description -->
      <HeroContent />

      <!-- Social Media Links -->
      <SocialMediaContainer />
    </div>

    <!-- Call to Action (Mobile only) -->
    <CallToAction />
  </section>
</template>
