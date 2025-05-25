<template>
  <main>
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" />
    <UApp>
      <Navbar />
      <div class="flex min-h-screen-without-header">
        <slot />
      </div>
    </UApp>
  </main>
</template>
