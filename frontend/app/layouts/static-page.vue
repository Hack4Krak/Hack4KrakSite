<template>
  <NuxtLayout name="default">
    <HeroDesktopHeroBackground class="h-screen fixed top-(--ui-header-height)" />
    <UContainer class="flex flex-col justify-center items-center mt-10 lg:mt-30 bg-default rounded-t-2xl">
      <slot />
    </UContainer>
    <!-- The background spans the whole width, the footer itself stays in its container -->
    <div class="w-full mt-8 lg:mt-12 bg-default">
      <Footer />
    </div>
  </NuxtLayout>
</template>
