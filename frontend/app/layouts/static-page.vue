<template>
  <NuxtLayout name="default">
    <HeroDesktopHeroBackground class="h-screen fixed top-(--ui-header-height)" />
    <UContainer class="flex flex-col justify-center items-center mt-10 lg:mt-30 mb-5 bg-default rounded-2xl">
      <slot />
      <Footer class="mt-8 lg:mt-12" />
    </UContainer>
  </NuxtLayout>
</template>
