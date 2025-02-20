<script setup lang="ts">

</script>

<template>
  <div class="flex flex-col m-5 md:m-10 gap-5">
    <h2 class="text-2xl font-bold">
      Czas do początku wydarzenia
    </h2>
    <span class="w-full text-center text-6xl font-bold">21 : 37 : 69</span>
    <h2 class="text-2xl font-bold mt-5">
      Wyślij flagę
    </h2>
    <UInput :ui="{ base: 'h-12' }" placeholder="hack4KrakCTF{...}" :disabled="true" />
    <ElevatedButton message="Wyślij" class="w-40" />
  </div>
</template>
