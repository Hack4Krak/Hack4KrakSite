<template>
  <NuxtLayout name="default">
    <div class="min-h-[calc(100vh-var(--ui-header-height))]">
      <div class="flex flex-col md:flex-row justify-center items-center bg-gray-100 dark:bg-zinc-900 rendering-pixelated bg-[url(assets/img/background.webp)] bg-cover bg-center min-h-[calc(100vh-var(--ui-header-height))]">
        <div class="my-5 space-y-4 sm:min-w-90 min-w-70 bg-default p-8 rounded-2xl">
          <slot class="sm:w-90 w-60" />
        </div>
      </div>
    </div>
  </NuxtLayout>
</template>
