<template>
  <main>
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" class="hidden" />
    <UApp>
      <Navbar />
      <slot />
    </UApp>
  </main>
</template>
