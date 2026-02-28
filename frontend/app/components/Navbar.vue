<script setup lang="ts">
import { NAVBAR_ITEMS } from '~~/content/navbar'

const { data: isLoggedIn } = useAuth('/auth/status', {
  redirect: 'error',
  onResponseError: undefined,
})

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

function openCommandPalette() {
  useCommandPaletteState().value = true
}
</script>

<template>
  <UHeader :ui="{ container: 'max-w-full' }" class="bg-default">
    <template #title>
      <img src="~/assets/img/logo.svg" class="h-(--spacing-icon-lg)" alt="Hack4Krak logo">
    </template>

    <UNavigationMenu v-bind="navigationMenuProperties" />

    <template #right>
      <button
        class="hidden lg:flex items-center gap-2 px-3 py-1.5 text-sm text-muted border border-default rounded-md hover:bg-elevated cursor-pointer transition-colors"
        aria-label="Wyszukaj strony (Ctrl+K)"
        @click="openCommandPalette"
      >
        <UIcon name="mdi:magnify" class="text-base" />
        <span>Szukaj...</span>
        <span class="ml-2 inline-flex items-center gap-0.5">
          <UKbd value="meta" size="sm" />
          <UKbd value="K" size="sm" />
        </span>
      </button>

      <NuxtLink to="/login" class="ml-4 text-md font-semibold flex justify-end" :aria-label="isLoggedIn ? 'Otwórz panel' : 'Zaloguj się'">
        <UIcon :name="isLoggedIn ? 'pixelarticons:user' : 'pixelarticons:login'" class="icon-md lg:hidden" />

        <span class="hidden lg:inline">
          {{ isLoggedIn ? "Otwórz panel" : "Zaloguj się" }}
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
