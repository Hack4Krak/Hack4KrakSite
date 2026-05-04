<template>
  <NuxtLayout name="default">
    <div
      class="flex w-full flex-1 bg-default sm:flex-col sm:items-center sm:justify-center sm:bg-[url(assets/img/background.webp)] bg-cover endering-pixelated"
    >
      <div class="min-h-full w-full bg-default px-6 py-6 sm:my-5 sm:min-h-0 sm:w-auto sm:min-w-90 sm:space-y-4 sm:rounded-2xl sm:p-8">
        <slot />
      </div>
    </div>
  </NuxtLayout>
</template>
