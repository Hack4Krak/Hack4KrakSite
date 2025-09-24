<template>
  <NuxtLayout name="default">
    <div
      class="flex flex-col justify-center items-center w-full min-h-[inherit]
        bg-[url(assets/img/background.webp)] bg-cover rendering-pixelated"
    >
      <div class="my-5 p-8 rounded-2xl space-y-4 sm:min-w-90 min-w-70 bg-default">
        <slot />
      </div>
    </div>
  </NuxtLayout>
</template>
