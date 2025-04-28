<script setup lang="ts">
import CallToAction from '~/components/hero/CallToAction.vue'
import DesktopHeroBackground from '~/components/hero/DesktopHeroBackground.vue'
import MobileLogoSection from '~/components/hero/MobileLogoSection.vue'
import SocialMediaContainer from '~/components/hero/SocialMediaContainer.vue'
</script>

<template>
  <section
    class="hero-section lg:h-[calc(100vh-var(--ui-header-height))] flex flex-col items-center
            w-full mb-12 gap-y-12 lg:gap-0"
  >
    <DesktopHeroBackground hydrate-on-visible class="h-3/4" />
    <MobileLogoSection hydrate-on-visible />
    <div
      class="hero-content flex-1 flex flex-col lg:flex-row items-center bg-transparent
               gap-y-12 lg:gap-0 place-content-between xl:w-(--ui-container)"
    >
      <HeroContent />
      <SocialMediaContainer />
    </div>
    <CallToAction hydrate-on-visible class="lg:hidden" />
  </section>
</template>
