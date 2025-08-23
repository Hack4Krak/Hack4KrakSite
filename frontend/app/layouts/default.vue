<template>
  <main class="h-full flex flex-col">
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" />
    <UApp>
      <Navbar />
      <slot />
    </UApp>
  </main>
</template>
