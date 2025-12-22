<script setup lang="ts">
import { NAVBAR_ITEMS } from '~~/content/navbar'

const { data: username } = useAsyncData('username', async () => useNavbarUser())

async function logout() {
  await useAuth('/auth/logout', {
    method: 'POST',
  })

  await refreshNuxtData()
  await navigateTo('/login')
}

const navigationMenuProperties = computed(() => ({
  'content-orientation': 'vertical' as const,
  'items': NAVBAR_ITEMS,
  'variant': 'link' as const,
  'ui': {
    linkLabel: 'hover:underline underline-offset-5 text-md',
    viewport: 'w-(--reka-navigation-menu-viewport-width)',
    childList: 'flex-col items-center',
    link: 'text-md text-default data-active:text-secondary',
    list: 'gap-8',
  },
}))
</script>

<template>
  <UHeader :ui="{ container: 'max-w-full' }" class="bg-default">
    <template #title>
      <LogoWithText class="text-base" />
    </template>

    <UNavigationMenu v-bind="navigationMenuProperties" />

    <template #right>
      <NuxtLink to="/login" class="text-md font-semibold flex justify-end w-full" :aria-label="username ? 'Otwórz panel' : 'Zaloguj się'">
        <UIcon :name="username ? 'pixelarticons:user' : 'pixelarticons:login'" class="icon-md lg:hidden" />

        <span class="hidden lg:inline">
          <span v-if="!username">Zaloguj się</span>
          <div v-else class="flex gap-1.5 items-center">
            <span class="text-neutral-400 font-normal text-sm">{{ username }}</span>
            <UIcon name="pixelarticons:logout" class="icon-lg" @click="logout" />
          </div>
        </span>
      </NuxtLink>

      <div class="md:flex hidden flex-1 justify-end items-center" />
    </template>

    <template #toggle="{ open, toggle }">
      <button
        class="p-2 ml-2 cursor-pointer flex justify-center lg:hidden"
        aria-label="Przełącz nawigacje mobilną"
        data-testid="mobile-menu-toggle"
        @click="toggle"
      >
        <UIcon :name="open ? 'pixelarticons:close' : 'pixelarticons:menu'" class="icon-lg" />
      </button>
    </template>

    <template #body>
      <UNavigationMenu v-bind="navigationMenuProperties" orientation="vertical" class="-mx-2.5" />
    </template>
  </UHeader>
</template>
