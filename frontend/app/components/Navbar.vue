<script setup lang="ts">
const navbarItems = {
  Kontakt: '#',
  Discord: 'https://discord.gg/ASPqckzEd8',
  Ranking: '#',
  Regulamin: '#',
}

const isExpanded = ref(false)
const router = useRouter()

watch(() => router.currentRoute.value, () => {
  isExpanded.value = false
})
</script>

<template>
  <nav class="bg-black font-logotype absolute w-full top-0 z-20">
    <div class="max-w-7xl mx-auto p-3 flex justify-between items-center">
      <div class="flex items-center justify-between space-x-5">
        <a class="text-2xl font-bold text-gray-800 dark:text-white flex items-center" href="/">
          <Logo /><span class="block sm:hidden">Hack4Krak</span>
        </a>
        <div class="hidden space-x-10 sm:block">
          <ULink
            v-for="(url, text) in navbarItems" :key="text" :to="url"
            class="text-gray-800 dark:text-white hover:text-blue-500"
          >
            {{ text }}
          </ULink>
        </div>
      </div>
      <button class="hidden sm:block px-2 py-1 rounded-lg drop-shadow-[-3px_5px_#1A51AC] bg-amber-400 font-pixelify text-blue-800">
        <span class="drop-shadow-[0_2px_5px_#333333dd]">START GRY</span>
      </button>
      <button class="sm:hidden text-3xl flex items-center" @click="isExpanded = !isExpanded">
        <Icon :name="isExpanded ? 'mdi:close' : 'mdi:hamburger-menu'" width="24" height="24" />
      </button>
    </div>
    <div :class="isExpanded ? 'h-[80vh]' : '-translate-y-[110%] h-0'" class="flex sm:hidden overflow-hidden flex-col justify-between duration-300 transition-all">
      <div class="flex flex-col">
        <ULink
          v-for="(url, text) in navbarItems" :key="text" :to="url"
          class="text-gray-800 dark:text-white hover:text-blue-500 p-3 text-2xl self-center"
        >
          {{ text }}
        </ULink>
      </div>

      <button class="w-[50vw] px-2 mb-15 py-3 rounded-lg drop-shadow-[0_5px_#1A51AC] bg-[#EDB647] font-pixelify text-xl text-blue-800 self-center">
        <span class="drop-shadow-[0_2px_5px_#333333dd]">START GRY</span>
      </button>
    </div>
  </nav>
</template>
