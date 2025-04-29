<template>
  <main>
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" class="hidden" />
    <UApp>
      <Navbar />
      <HeroDesktopHeroBackground class="h-screen fixed" />
      <UContainer class="flex flex-col justify-center items-center mt-10 lg:mt-30 mb-5 bg-default rounded-2xl">
        <slot />
      </UContainer>
    </UApp>
  </main>
</template>
