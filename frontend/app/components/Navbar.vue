<script setup lang="ts">
import { NAVBAR_ITEMS } from '~~/content/navbar'

const [DefineNavbarTemplate, ReuseNavbarTemplate] = createReusableTemplate()

const isMobileMenuOpen = ref(false)

const { data: isLoggedIn } = useAuth('/auth/status', {
  redirect: 'error',
  onResponseError: undefined,
})

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
  <DefineNavbarTemplate v-slot="{ orientation }">
    <div class="flex w-full">
      <div class="md:flex hidden flex-1 items-center">
        <NuxtLink to="/">
          <LogoWithText />
        </NuxtLink>
      </div>
      <div>
        <UNavigationMenu
          content-orientation="vertical"
          :orientation="orientation"
          :items="NAVBAR_ITEMS" variant="link" class="w-full" color="error"
          :ui="{
            linkLabel: 'hover:underline underline-offset-5 text-md',
            viewport: 'w-(--reka-navigation-menu-viewport-width)',
            childList: 'flex-col items-center',
            link: 'text-md text-default data-active:text-secondary',
            list: 'gap-8',
          }"
        />
      </div>
      <div class="md:flex hidden flex-1 justify-end items-center">
        <NuxtLink to="/login" class="text-md font-semibold">
          {{ isLoggedIn ? "Otwórz panel" : "Zaloguj się" }}
        </NuxtLink>
      </div>
    </div>
  </DefineNavbarTemplate>

  <UContainer class="sticky top-0 max-w-full font-sans bg-default z-20 print:hidden">
    <div class="hidden md:flex justify-center items-center h-(--ui-header-height)">
      <ReuseNavbarTemplate orientation="horizontal" />
    </div>

    <!-- Mobile Navigation -->
    <div class="md:hidden flex gap-4 py-2">
      <NuxtLink to="/" class="flex items-center">
        <LogoWithText />
      </NuxtLink>
      <NuxtLink to="/login" class="flex items-center text-md font-semibold ml-auto" :aria-label="isLoggedIn ? 'Otwórz panel' : 'Zaloguj się'">
        <UIcon :name="isLoggedIn ? 'pixelarticons:user' : 'pixelarticons:login'" class="icon-md" />
      </NuxtLink>
      <button
        class="p-2 cursor-pointer flex justify-center"
        aria-label="Przełącz nawigacje mobilną"
        data-testid="mobile-menu-toggle"
        @click="toggleMobileMenu"
      >
        <UIcon :name="isMobileMenuOpen ? 'pixelarticons:close' : 'pixelarticons:menu'" class="icon-lg" />
      </button>
    </div>

    <Transition
      enter-from-class="opacity-0 translate-x-[100%]"
      leave-to-class="opacity-0 translate-x-[100%]"
      enter-active-class="transition duration-200"
      leave-active-class="transition duration-200"
      hydrate-on-media-query="(max-width: 768px)"
    >
      <div v-if="isMobileMenuOpen" class="my-2 md:hidden h-screen [&>a]:text-5xl ">
        <ReuseNavbarTemplate orientation="vertical" />
      </div>
    </Transition>
  </UContainer>
</template>
