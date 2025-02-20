<script setup lang="ts">
const props = defineProps<{
  links: { name: string, url: string }[]
}>()

const route = useRoute()

function is_current(checked_route: string) {
  return route.path === checked_route
}
</script>

<template>
  <div class="w-1/8 flex flex-col p-5 pl-7 bg-black border-r-1 border-r-gray-600 font-pixelify">
    <ul class="flex flex-col gap-2 text-lg">
      <li class="hover:underline w-min">
        <NuxtLink to="/admin" :class="is_current('/admin') ? 'font-bold underline' : ''">
          Home
        </NuxtLink>
      </li>
      <li v-for="link in props.links" :key="link.url" class="hover:underline w-min">
        <NuxtLink :to="link.url" :class="is_current(link.url) ? 'font-bold underline' : ''">
          {{ link.name }}
        </NuxtLink>
      </li>
    </ul>
  </div>
</template>
