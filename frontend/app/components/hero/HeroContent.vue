<template>
  <div class="relative lg:-top-20">
    <h1 class="font-bold pr-4 stroked-text-1 text-3xl lg:text-9xl">
      <slot name="title" />
    </h1>
    <p class="hero-description lg:text-2xl whitespace-pre-line">
      <slot name="content" />
    </p>
  </div>
</template>
