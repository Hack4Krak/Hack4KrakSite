<script setup lang="ts">
import file from 'assets/faq.json?raw'

const content = JSON.parse(file)

const items = ref(content)
</script>

<template>
  <div class="mx-7 md:mx-14">
    <div class="font-semibold mt-14 text-5xl text-yellow-400">
      FAQ
    </div>
    <div class="font-light mt-3 text-2xl">
      Czyli najczęśniej zadawane pytania, które nigdy nie padły :O
    </div>
    <UAccordion class="cursor-pointer mt-8" type="multiple" trailing-icon="i-material-symbols-keyboard-arrow-down-rounded" :items="items" />
  </div>
</template>
