<template>
  <main>
    <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" />
    <UApp>
      <Navbar />
      <div class="flex min-h-[calc(100vh-var(--ui-header-height))]">
        <slot />
      </div>
    </UApp>
  </main>
</template>
