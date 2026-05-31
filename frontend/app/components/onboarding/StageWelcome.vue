<template>
  <section class="flex min-h-[20rem] flex-col items-center justify-center gap-4 text-center sm:min-h-[22rem]">
    <div class="w-32 md:w-36">
      <img
        src="~/assets/img/error_dragon.webp"
        alt="Smok Wawelski wita w Hack4Krak"
        class="w-full rendering-pixelated select-none animate-[float_4s_ease-in-out_infinite]"
        draggable="false"
      >
    </div>
    <h1 class="text-2xl font-bold leading-tight sm:text-3xl">
      Witaj w <span class="text-primary">Hack4Krak</span>
    </h1>
    <p class="max-w-md text-sm text-muted sm:text-base">
      Zanim ruszą zawody, poznajmy się trochę — kilka pytań zajmie chwilę.
    </p>
    <p class="text-muted max-w-md text-sm">
      Twoje odpowiedzi są widoczne wyłącznie dla organizatorów.
    </p>
  </section>
</template>

<style scoped>
@keyframes float {
  0%,
  100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-6px);
  }
}
</style>
