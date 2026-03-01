<script setup lang="ts">
defineSlots<{
  title: () => VNode | VNode[]
  content: () => VNode | VNode[]
}>()
</script>

<template>
  <section
    class="lg:h-screen-without-header flex flex-col items-center w-full gap-y-12 pt-6 lg:pt-0 lg:gap-0"
  >
    <LazyHeroDesktopHeroBackground hydrate-on-visible class="h-3/4" />
    <LazyHeroMobileLogoSection hydrate-on-visible />
    <div
      class="flex-1 flex flex-col lg:flex-row items-center bg-transparent
             gap-y-12 lg:gap-0 place-content-between xl:w-(--ui-container) px-6"
    >
      <HeroContent>
        <template #title>
          <slot name="title" />
        </template>
        <template #content>
          <slot name="content" />
        </template>
      </HeroContent>
      <HeroSocialMediaContainer />
    </div>
    <LazyHeroCallToAction hydrate-on-visible class="lg:hidden" />
  </section>
</template>
