<script setup lang="ts">
const isMobileMenuOpen = ref(false)
function toggleMobileMenu() {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}

// Hide navbar on button press
const router = useRouter()
watch(() => router.currentRoute.value, () => {
  isMobileMenuOpen.value = false
})

const items = [
  [
    {
      label: 'Kontakt',
      to: '/contact',
    },
    {
      label: 'Ranking',
      to: '/ranking',
    },
    {
      label: 'Regulamin',
      to: '/rules',
    },
    {
      label: 'Zadania',
      to: '/tasks',
    },
    {
      label: 'Discord',
      to: 'https://discord.gg/ASPqckzEd8',
      target: '_blank',
    },
    {
      label: 'FAQ',
      target: '/faq',
    },
  ],
  [
    {
      slot: 'button',
      to: '/login',
    },
  ],
]
</script>

<template>
  <UContainer class="top-0 z-10 sticky max-w-full border-b-1 border-neutral-600 font-pixelify bg-(--ui-bg)">
    <div class="mx-auto flex items-center">
      <NuxtLink to="/" class="flex items-center space-x-2 py-3">
        <Logo class="size-10 text-black dark:text-white" />
        <h1 class="md:hidden text-2xl font-semibold">
          Hack4Krak
        </h1>
      </NuxtLink>

      <!-- Desktop Navigation -->
      <UNavigationMenu
        :items="items" variant="link" class="hidden md:flex w-full"
        :ui="{ linkLabel: 'text-lg hover:underline underline-offset-5 text-(--ui-bg) dark:text-white' }"
      >
        <template #button>
          <ElevatedButton message="START GRY" />
        </template>
      </UNavigationMenu>

      <button class="md:hidden p-2 ml-auto cursor-pointer" @click="toggleMobileMenu">
        <Icon :name="isMobileMenuOpen ? 'mdi:close' : 'mdi:hamburger-menu'" size="24" />
      </button>
    </div>

    <!-- Mobile Navigation -->
    <Transition
      enter-from-class="opacity-0 translate-x-[100%]"
      leave-to-class="opacity-0 translate-x-[100%]"
      enter-active-class="transition duration-200"
      leave-active-class="transition duration-200"
    >
      <div v-if="isMobileMenuOpen" class="md:hidden h-screen [&>a]:text-5xl ">
        <USeparator class="mb-2" />
        <UNavigationMenu
          :items="items"
          orientation="vertical"
          variant="link"
          class="w-full text-3xl"
          :ui="{ link: 'text-lg text-white text-black dark:text-white' }"
        >
          <template #button>
            <div class="items-center justify-center text-center w-full">
              <ElevatedButton message="START GRY" class="w-70 mt-2" />
            </div>
          </template>
        </UNavigationMenu>
      </div>
    </Transition>
  </UContainer>
</template>
