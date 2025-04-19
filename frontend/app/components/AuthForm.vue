<template>
  <div class="flex flex-col md:flex-row justify-center items-center bg-gray-100 dark:bg-zinc-900 rendering-pixelated bg-[url(assets/img/background.webp)] bg-cover bg-center h-full">
    <div class="space-y-4 w-90 bg-default p-8 rounded-2xl">
      <slot />
    </div>
  </div>
</template>
