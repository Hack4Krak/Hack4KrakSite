<template>
  <button class="elevated-button bg-primary-500">
    <span
      class="block text-md font-bold px-4 py-2 shadow-cream -translate-x-1 -translate-y-1
      bg-inverted text-inverted border-dark border-2
      duration-300 hover:-translate-y-1.5 hover:-translate-x-1.5 hover:cursor-pointer
      active:translate-x-0 active:translate-y-0"
    >
      <slot />
    </span>
  </button>
</template>
