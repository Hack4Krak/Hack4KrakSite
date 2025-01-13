<template>
  <NuxtLoadingIndicator />
  <MainNavbar />
  <div class="min-h-[calc(100vh-var(--header-height))]">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
</template>
