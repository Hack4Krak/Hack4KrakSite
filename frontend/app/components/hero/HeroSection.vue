<template>
  <section
    class="lg:h-[calc(100vh-var(--ui-header-height))] flex flex-col items-center
           w-full mb-12 gap-y-12 lg:gap-0"
  >
    <HeroDesktopHeroBackground hydrate-on-visible class="h-3/4" />
    <HeroMobileLogoSection hydrate-on-visible />
    <div
      class="flex-1 flex flex-col lg:flex-row items-center bg-transparent
             gap-y-12 lg:gap-0 place-content-between xl:w-(--ui-container)"
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
    <HeroCallToAction hydrate-on-visible class="lg:hidden" />
  </section>
</template>
