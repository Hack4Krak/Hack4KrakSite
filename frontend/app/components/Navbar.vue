<script setup lang="ts">
const isMobileMenuOpen = ref(false)
function toggleMobileMenu() {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}

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
      label: 'Discord',
      to: 'https://discord.gg/ASPqckzEd8',
      target: '_blank',
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
  <UContainer class="top-0 z-1 sticky bg-white dark:bg-black max-w-full">
    <div class="mx-auto flex items-center">
      <NuxtLink to="/" class="flex items-center space-x-2">
        <Logo class="size-10 text-black dark:text-white" />
        <h1 class="md:hidden">
          Hack4Krak
        </h1>
      </NuxtLink>

      <!-- Desktop Navigation -->
      <UNavigationMenu :items="items" class="hidden md:flex w-full" variant="link">
        <template #button>
          <ElevatedButton message="START GRY" />
        </template>
      </UNavigationMenu>

      <button class="md:hidden p-2 ml-auto cursor-pointer" @click="toggleMobileMenu">
        <Icon :name="isMobileMenuOpen ? 'i-lucide-menu' : 'i-lucide-minimize-2'" size="20" />
      </button>
    </div>

    <!-- Mobile Navigation -->
    <transition
      enter-from-class="opacity-0 translate-x-[100%]"
      leave-to-class="opacity-0 translate-x-[100%]"
      enter-active-class="transition duration-200"
      leave-active-class="transition duration-200"
    >
      <div v-if="isMobileMenuOpen" class="md:hidden h-screen overflow-hidden">
        <USeparator />
        <UNavigationMenu
          :items="items"
          orientation="vertical"
          variant="link"
          class="w-full items-center justify-center text-center text-3xl"
        >
          <template #button>
            <ElevatedButton message="START GRY" />
          </template>
        </UNavigationMenu>
      </div>
    </transition>
  </UContainer>
</template>
