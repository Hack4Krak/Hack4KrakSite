<template>
  <article class="relative border-2 border-primary bg-default p-1">
    <div aria-hidden="true">
      <span class="absolute -left-0.5 -top-0.5 size-3 border-b-2 border-r-2 border-primary bg-default" />
      <span class="absolute -right-0.5 -top-0.5 size-3 border-b-2 border-l-2 border-primary bg-default" />
      <span class="absolute -bottom-0.5 -left-0.5 size-3 border-r-2 border-t-2 border-primary bg-default" />
      <span class="absolute -bottom-0.5 -right-0.5 size-3 border-l-2 border-t-2 border-primary bg-default" />

      <span class="absolute -top-0.5 right-66 hidden h-3 w-8 border-x-2 border-b-2 border-primary bg-default lg:block" />
      <span class="absolute -bottom-0.5 right-66 hidden h-3 w-8 border-x-2 border-t-2 border-primary bg-default lg:block" />
    </div>

    <div class="relative z-10">
      <slot />
    </div>
  </article>
</template>
