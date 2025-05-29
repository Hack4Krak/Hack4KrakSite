<script setup lang="ts">
const { data } = await useApi('/event/info')

const futureDate = computed(() => new Date(data.value?.start_date ?? Date.now()))

function redirectToTasksPage() {
  navigateTo(`/tasks`)
}
</script>

<template>
  <div class="content-center text-balance font-roboto items-center justify-center min-h-[calc(100vh-var(--ui-header-height))]">
    <div class="flex content-center items-center justify-center flex-col-reverse lg:flex-row">
      <div class="flex flex-col max-lg:items-center">
        <p class="text-yellow-500 my-5 sm:text-5xl xl:text-6xl text-4xl font-bold">
          Skąd ten pośpiech?
        </p>
        <p class="xl:text-4xl lg:text-2xl sm:text-3xl text-2xl mt-5 tracking-wide">
          CTF jeszcze się nie rozpoczął!
        </p>
        <p class="xl:text-4xl lg:text-2xl sm:text-3xl text-2xl mt-3 ">
          Czas do początku wydarzenia:
        </p>
        <Timer class="mt-10" :target="futureDate" @complete="redirectToTasksPage" />
      </div>
      <div class="w-3/4 mb-10 lg:w-150 lg:mb-0 lg:ml-10">
        <img class="w-full rendering-pixelated" src="../assets/img/error_dragon.webp" alt="Dragon of (t)error">
      </div>
    </div>
  </div>
</template>
