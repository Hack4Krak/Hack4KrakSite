<script setup lang="ts">
import file from 'assets/faq.json?raw'
const content = JSON.parse(file);
const items = ref(content)

</script>

<template>
    <div class="font-semibold ml-14 mt-8 text-5xl text-yellow-400">FAQ</div>
    <div class="font-light ml-14 mt-3 text-2xl">Czyli najczęśniej zadawane pytania, które nigdy nie padły :O</div>
    <UAccordion class='ml-8 md:ml-24 lg:ml-24 mt-16 w-5/6 md:w-[calc(100%-16rem)]' type='multiple' trailing-icon="i-lucide-arrow-down" :items="items" />
</template>