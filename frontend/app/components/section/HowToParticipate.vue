<script setup lang="ts">
import type { TabsItem } from '#ui/components/Tabs.vue'
import PARTICIPATE_CONTENT from '~~/content/how-to-participate.json'

const items: TabsItem[] = PARTICIPATE_CONTENT.tab_items

// slot must be defined or else ts gonna yell :3
const filteredItems = computed(() => items.filter(item => item.slot))
</script>

<template>
  <div class="flex flex-col border-neutral-400 border-2 bg-neutral-400/10 p-6 text-center justify-center">
    <p class="text-xl lg:text-2xl font-bold mt-6 mb-2">
      Jak zapisać się na wydarzenie?
    </p>
    <UTabs :items="items" variant="link" class="gap-4 w-full" color="neutral" :ui="{ trigger: 'flex-1' }" :unmount-on-hide="false">
      <template v-for="item in filteredItems" :key="item.slot" #[item.slot!]>
        <OrderedList :items="item.description" />
      </template>
    </UTabs>
  </div>
</template>
