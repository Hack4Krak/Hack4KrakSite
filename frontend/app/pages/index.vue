
<template>
  <ColorScheme>
  <USelect
              v-model="$colorMode.preference"
              :items="['system', 'light', 'dark']"
  />
</ColorScheme>
  <div class="px-4 py-4 bg-gray-100 dark:bg-zinc-900">
    <div class="flex justify-center items-center h-screen">
      <logo class="fill-current text-black dark:text-white max-h-96 max-w-96 mr-12" src="img/logo.svg" alt="Hack4Krak logo" />
      <div class="text-black dark:text-white font-[25] text-[300px] flex justify-center items-center">
        |
      </div>
      <div class="ml-10 text-black dark:text-white">
        <div class="mb-10 text-8xl font-semibold">
          Hack4Krak
        </div>
        <div class="flex text-4xl text-black font-bold dark:text-white">
          Strona w trakcie tworzenia
        </div>
        <div>
          <div class="mt-10 flex text-4xl items-center">
            <a class="h-fit w-60" href="https://discord.gg/fJyZT6EPcY">
              <Icon name="logos:discord" class="[&>path]:fill-black dark:[&>path]:fill-white" mode="svg" alt="Official logo of Discord"/>
            </a>
            <a class="h-fit w-12 text-black dark:text-white" href="https://github.com/Hack4Krak/Hack4KrakSite/">
              <Icon name="mdi:github"/>
            </a>
            <a class="h-fit w-32 text-black dark:text-white" href="https://github.com/Hack4Krak/Hack4KrakSite/">
              <Icon name="octicon:logo-github-24"/>
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
</script>