<script setup lang="ts">
import { NAVBAR_ITEMS } from '~/content/navbar'

const [DefineNavbarTemplate, ReuseNavbarTemplate] = createReusableTemplate()

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
  <DefineNavbarTemplate>
    <UNavigationMenu
      :items="NAVBAR_ITEMS" variant="link" class="w-full" color="error"
      :ui="{ linkLabel: ' hover:underline underline-offset-5', link: 'text-md text-default data-active:text-primary', list: 'gap-4' }"
    >
      <template #logo>
        <div class="md:flex hidden">
          <LogoWithText />
        </div>
      </template>

      <template #button>
        <ElevatedButtonLink to="/panel">
          Zaloguj się!
        </ElevatedButtonLink>
      </template>
    </UNavigationMenu>
  </DefineNavbarTemplate>

  <UContainer class="sticky top-0 max-w-full font-sans bg-default z-20">
    <div class="hidden md:flex items-center place-content-between h-(--ui-header-height)">
      <ReuseNavbarTemplate />
    </div>

    <!-- Mobile Navigation -->
    <div class="md:hidden flex pt-4">
      <NuxtLink to="/" class="flex items-center mb-4">
        <LogoWithText />
      </NuxtLink>
      <button
        class="p-2 ml-auto cursor-pointer flex justify-center"
        aria-label="Przełącz nawigacje mobilną"
        data-testid="mobile-menu-toggle"
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
      <div v-if="isMobileMenuOpen" class="-my-5 md:hidden h-screen [&>a]:text-5xl ">
        <ReuseNavbarTemplate orientation="vertical" />
      </div>
    </Transition>
  </UContainer>
</template>
