<script setup lang="ts">

</script>

<template>
  <div class="flex flex-col m-5 gap-5">
    <h2 class="text-2xl font-bold">
      Czas do początku wydarzenia
    </h2>
    <span class="w-full text-center text-5xl font-bold">21 : 37 : 69</span>
  </div>
  <div class="flex flex-col m-5 gap-5">
    <h2 class="text-2xl font-bold">
      Wyślij flagę
    </h2>
    <UInput :ui="{ base: 'h-10' }" placeholder="hack4KrakCTF{...}" :disabled="true" />
    <ElevatedButton message="Wyślij" class="w-40" />
  </div>
</template>
