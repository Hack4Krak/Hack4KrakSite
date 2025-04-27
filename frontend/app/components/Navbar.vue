<script setup lang="ts">
import { NAVBAR_ITEMS } from '~/content/navbar'

const isMobileMenuOpen = ref(false)

function toggleMobileMenu() {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}

// Hide navbar on button press
const router = useRouter()
watch(() => router.currentRoute.value, () => {
  isMobileMenuOpen.value = false
})
</script>

<template>
  <UContainer class="sticky top-0 max-w-full font-sans bg-dark z-20">
    <div class="hidden md:flex items-center place-content-between h-(--ui-header-height)">
      <!-- Logo + Logotype -->
      <NuxtLink to="/" class="flex items-center">
        <Logo class="size-9 dark:text-white mr-4" />
        <p class="font-pixelify text-xl font-semibold">
          Hack4Krak
        </p>
      </NuxtLink>

      <!-- Desktop Navigation -->
      <UNavigationMenu
        content-orientation="vertical"
        :items="NAVBAR_ITEMS" variant="link" class="hidden md:flex justify-center"
        :ui="{ link: 'text-md hover:underline underline-offset-5 text-dark dark:text-bright', list: 'gap-4' }"
      />

      <ElevatedButtonLink to="/panel">
        Zaloguj się!
      </ElevatedButtonLink>
    </div>

    <!-- Mobile Navigation -->
    <div class="md:hidden flex pt-4">
      <NuxtLink to="/" class="flex items-center mb-4">
        <Logo class="size-9 dark:text-white mr-4" />
        <p class="font-pixelify text-xl font-semibold">
          Hack4Krak
        </p>
      </NuxtLink>
      <button
        class="p-2 ml-auto cursor-pointer flex justify-center" aria-label="Toogle navbar"
        @click="toggleMobileMenu"
      >
        <Icon :name="isMobileMenuOpen ? 'mdi:close' : 'mdi:hamburger-menu'" size="28" />
      </button>
    </div>

    <Transition
      enter-from-class="opacity-0 translate-x-[100%]"
      leave-to-class="opacity-0 translate-x-[100%]"
      enter-active-class="transition duration-200"
      leave-active-class="transition duration-200"
      hydrate-on-media-query="(max-width: 768px)"
    >
      <div v-if="isMobileMenuOpen" class="md:hidden h-screen [&>a]:text-5xl ">
        <USeparator class="mb-2" />
        <LazyUNavigationMenu
          :items="NAVBAR_ITEMS"
          orientation="vertical"
          variant="link"
          class="w-full text-3xl"
          :ui="{ link: 'text-lg text-bright dark:text-bright' }"
        >
          <template #button>
            <div class="items-center justify-center text-center w-full">
              <ElevatedButtonLink class="w-70 mt-2" to="/panel">
                Start gry!
              </ElevatedButtonLink>
            </div>
          </template>
        </LazyUNavigationMenu>
      </div>
    </Transition>
  </UContainer>
</template>
