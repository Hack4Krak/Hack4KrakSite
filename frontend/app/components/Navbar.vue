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
  <nav class="bg-black font-logotype fixed w-full top-0 z-1">
    <div class="max-w-full h-19 mx-auto py-3 px-10 flex justify-between items-center">
      <div class="flex items-center justify-between space-x-5">
        <a class="text-2xl font-bold text-white flex items-center" href="/">
          <Logo /><span class="block sm:hidden">Hack4Krak</span>
        </a>
        <div class="hidden space-x-10 sm:block">
          <ULink
            v-for="(url, text) in navbarItems" :key="text" :to="url"
            class="text-white hover:underline"
          >
            {{ text }}
          </ULink>
        </div>
      </div>
      <button class="hidden sm:block px-3 py-2 mb-1 rounded-lg drop-shadow-[0_5px_#1A51AC] hover:drop-shadow-[0_5px_#6CA3FF] bg-[#EDB647] hover:bg-[#FDC657] font-pixelify text-xl text-blue-800 self-center cursor-pointer duration-300 transition-all font-bold">
        <span>START GRY</span>
      </button>
      <button class="sm:hidden text-3xl flex items-center" @click="isExpanded = !isExpanded">
        <Icon :name="isExpanded ? 'mdi:close' : 'mdi:hamburger-menu'" width="24" height="24" />
      </button>
    </div>
    <div :class="isExpanded ? 'h-screen' : '-translate-y-[110%] h-0'" class="flex sm:hidden overflow-hidden flex-col justify-between duration-300 transition-all">
      <div class="flex flex-col">
        <ULink
          v-for="(url, text) in navbarItems" :key="text" :to="url"
          class="text-white hover:text-blue-500 p-3 text-2xl self-center"
        >
          {{ text }}
        </ULink>
      </div>

      <button class="w-[50vw] px-2 mb-55 py-3 rounded-lg drop-shadow-[0_5px_#1A51AC] bg-[#EDB647] font-pixelify text-xl text-blue-800 self-center cursor-pointer font-bold">
        <span>START GRY</span>
      </button>
    </div>
  </nav>
</template>
