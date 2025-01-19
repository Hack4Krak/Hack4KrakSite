<template>
  <NuxtLoadingIndicator />
  <Navbar />
  <div class="min-h-[calc(100vh-var(--spacing)*20)] text-black dark:text-white">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
</template>
