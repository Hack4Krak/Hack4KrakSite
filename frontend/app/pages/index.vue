<script setup lang="ts">
</script>

<template>
  <div class="px-4 py-4 bg-gray-100 dark:bg-zinc-900">
    <div class="flex justify-center items-center h-screen">
      <logo class="fill-current text-black dark:text-white max-h-96 max-w-96 mr-12" src="img/logo.svg" alt="Hack4Krak logo" />
      <USeparator orientation="vertical" class="h-88" size="lg" :ui="{ border: 'border-black dark:border-white' }" />
      <div class="ml-10 text-black dark:text-white">
        <div class="text-8xl font-semibold">
          Hack4Krak
        </div>
        <div class="mt-4 mb-10 h-fit w-[600px] text-lg text-black/80 dark:text-white/80">
          Organization responsible for managing the largest Capture the Flag (CTF) competition for high school students in Kraków, Poland.
        </div>
        <div class="flex text-4xl font-medium text-black/90 dark:text-white/90">
          Strona w trakcie tworzenia
        </div>
        <div>
          <div class="mt-10 flex text-4xl items-center">
            <a class="h-fit w-32 mr-32" href="https://discord.gg/ASPqckzEd8">
              <Icon name="logos:discord" class="[&>path]:fill-black dark:[&>path]:fill-white" mode="svg" alt="Official logo of Discord" />
            </a>
            <a class="h-fit w-12 text-black dark:text-white" href="https://github.com/Hack4Krak/Hack4KrakSite/">
              <Icon name="mdi:github" />
            </a>
            <a class="h-fit w-24 text-black dark:text-white" href="https://github.com/Hack4Krak/Hack4KrakSite/">
              <Icon name="octicon:logo-github-24" />
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
