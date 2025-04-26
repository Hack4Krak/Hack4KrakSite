<template>
  <div class="relative tile bor tile border border-neutral-600 bg-default rounded-3xl">
    <div class="overflow-hidden w-full h-full">
      <div class="h-full rounded-3xl">
        <slot>
          <Placeholder class="w-full h-full" />
        </slot>
      </div>
    </div>
  </div>
</template>
