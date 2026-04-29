<template>
  <NuxtLayout name="default">
    <div
      class="flex flex-col justify-center items-center w-full flex-1
        bg-[url(assets/img/background.webp)] bg-cover rendering-pixelated"
    >
      <div class="my-8 p-8 sm:min-w-96 min-w-72 bg-default border-2 border-surface-muted shadow-[4px_4px_0_var(--color-primary)]">
        <slot />
      </div>
    </div>
  </NuxtLayout>
</template>
