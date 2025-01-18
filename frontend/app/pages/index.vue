<script setup lang="ts">
const items = [
  'https://picsum.photos/640/320?random=1',
  'https://picsum.photos/640/320?random=2',
  'https://picsum.photos/640/320?random=3',
  'https://picsum.photos/640/320?random=4',
  'https://picsum.photos/640/320?random=5',
  'https://picsum.photos/640/320?random=6',
]
</script>

<template>
  <div class="px-4 py-4 dark:bg-black">
    <div class="flex justify-center items-center h-screen">
      <logo class="fill-current text-black dark:text-white max-h-96 max-w-96 mr-12" src="img/logo.svg" alt="Hack4Krak logo" />
      <div class="text-black dark:text-white font-[25] text-[300px] flex justify-center items-center">
        |
      </div>
      <div class="ml-10 text-black dark:text-white">
        <div class="mb-10 text-8xl font-semibold">
          Hack4Krak
        </div>
        <div class="flex text-4xl text-black/75 dark:text-black/0">
          Strona w trakcie tworzenia
        </div>
        <div>
          <div class="mt-10 flex text-4xl items-center">
            <a class="h-fit w-60" href="#">
              <img src="public/img/discord-mark-black.svg" alt="Official logo of Discord">
            </a>
            <a class="hidden dark:block h-fit w-60" href="#">
              <img src="public/img/discord-logo-white.svg" alt="Official logo of Discord">
            </a>
            <a class="ml-16 h-fit w-16" href="#">
              <img src="public/img/github-mark.svg" alt="Offical intertocat logo of GitHub">
            </a>
            <a class="hidden dark:block ml-16 h-fit w-16" href="#">
              <img src="public/img/github-mark-white.svg" alt="Offical intertocat logo of GitHub">
            </a>
            <a class="h-fit w-32" href="#">
              <img class="dark:img" src="public/img/GitHub_Logo.png" alt="Official logo of GitHub">
            </a>
            <a class="hidden dark:block h-fit w-32" href="#">
              <img class="dark:img" src="public/img/GitHub_Logo_White.png" alt="Official logo of GitHub">
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
