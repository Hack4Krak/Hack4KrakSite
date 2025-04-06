<script setup lang="ts">
import file from 'assets/faq.json'
</script>

<template>
  <div class="mx-7 md:mx-14">
    <div class="font-semibold mt-14 text-5xl text-yellow-400">
      FAQ
    </div>
    <div class="font-light mt-3 text-2xl">
      Czyli najczęśniej zadawane pytania, które nigdy nie padły :O
    </div>
    <UAccordion
      class="cursor-pointer mt-8" type="multiple" trailing-icon="i-material-symbols-keyboard-arrow-down-rounded" :ui="{
        label: 'text-lg',
        trailingIcon: 'text-yellow-500 size-6',
        trigger: 'cursor-pointer',
        body: 'cursor-default text-md font-light',
      }" :items="file"
    />
  </div>
</template>
