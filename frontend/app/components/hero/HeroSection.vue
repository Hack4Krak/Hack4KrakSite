<template>
  <section
    class="lg:h-[calc(100vh-var(--ui-header-height))] flex flex-col items-center
           w-full gap-y-12 lg:gap-0"
  >
    <HeroDesktopHeroBackground hydrate-on-visible class="h-3/4" />
    <HeroMobileLogoSection hydrate-on-visible />
    <div
      class="flex-1 flex flex-col lg:flex-row items-center bg-transparent
             gap-y-12 lg:gap-0 place-content-between xl:w-(--ui-container) px-6"
    >
      <HeroContent>
        <template v-for="(_, name) in $slots" #[name]="data">
          <slot :name="name" v-bind="data" />
        </template>
      </HeroContent>
      <HeroSocialMediaContainer />
    </div>
    <HeroCallToAction hydrate-on-visible class="lg:hidden" />
  </section>
</template>
