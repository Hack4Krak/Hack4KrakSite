<template>
  <main>
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" class="hidden" />
    <UApp>
      <slot />
    </UApp>
  </main>
</template>
