<script setup lang="ts">
useSeoMeta({
  title: 'Strona Główna',
})
</script>

<template>
  <div class="flex flex-col md:flex-row justify-center items-center">
    <Logo class="max-h-48 max-w-48 md:max-h-96 md:max-w-96 md:mr-12" />
    <USeparator orientation="vertical" class="h-88 md:block hidden" size="lg" :ui="{ border: 'border-black dark:border-white' }" />
    <div class="mt-5 md:mt-0 md:ml-10 text-center md:text-left dark:text-white">
      <h1 class="text-5xl md:text-8xl font-semibold">
        Hack4Krak
      </h1>
      <p class="mt-4 mb-10 text-base md:text-lg max-w-xs md:max-w-150">
        Organization responsible for managing the largest Capture the Flag (CTF) competition for high school students in Kraków, Poland.
      </p>
      <div class="mt-10 flex justify-center md:justify-start h-10 text-2xl md:text-4xl items-center ">
        <a class="mr-5" href="https://discord.gg/ASPqckzEd8" target="_blank">
          <Icon name="logos:discord" class="[&>path]:fill-black dark:[&>path]:fill-white hover:[&>path]:fill-(--ui-primary)" mode="svg" alt="Discord" />
        </a>
        <a class="group hover:text-(--ui-primary)" href="https://github.com/Hack4Krak/Hack4KrakSite/" target="_blank">
          <Icon name="mdi:github" class="mr-2" />
          <Icon name="octicon:logo-github-24" alt="GitHub" />
        </a>
      </div>
    </div>
  </div>
</template>
