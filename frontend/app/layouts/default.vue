<template>
  <main class="h-screen overflow-auto flex flex-col">
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" />
    <UApp>
      <Navbar />
      <slot />
    </UApp>
  </main>
</template>
