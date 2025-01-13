<script setup>
const navbarItems = {
  Github: 'https://github.com/Hack4Krak/Hack4KrakSite',
}
</script>

<template>
  <nav class="bg-white dark:bg-gray-800 font-logotype">
    <div class="max-w-7xl mx-auto py-3 flex justify-between items-center">
      <a
        class="text-2xl font-bold text-gray-800 dark:text-white flex items-center"
        href="/"
      ><MainLogo />Hack4Krak</a>
      <div class="md:flex space-x-4 px-3">
        <ULink
          v-for="(url, text) in navbarItems"
          :key="text"
          :to="url"
          class="text-gray-800 dark:text-white hover:text-blue-500"
        >{{ text }}</ULink>
      </div>
    </div>
  </nav>
</template>
