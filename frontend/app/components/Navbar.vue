<script setup lang="ts">
import { NAVBAR_ITEMS } from '~~/content/navbar'

const { data: user } = useAuth('/account/', {
  redirect: 'error',
  onResponseError: undefined,
})

const { data: registrationInformation } = await useApi('/event/registration', {
  key: 'landing-registration-info',
})

const registrationOpen = computed(() => registrationInformation.value?.is_open ?? false)

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

const userMenuItems = computed(() => [
  [
    {
      label: user.value?.username ?? 'Użytkownik',
      type: 'label' as const,
    },
  ],
  [
    {
      label: 'Panel',
      icon: 'pixelarticons:dashboard',
      onSelect: () => navigateTo('/panel'),
    },
    {
      label: 'Profil',
      icon: 'pixelarticons:user',
      onSelect: () => navigateTo('/panel/profile'),
    },
  ],
  [
    {
      label: 'Wyloguj się',
      icon: 'pixelarticons:logout',
      color: 'error' as const,
      onSelect: () => useSession().logout(),
    },
  ],
])
</script>

<template>
  <UHeader :ui="{ container: 'max-w-full' }" class="bg-default">
    <template #title>
      <img src="~/assets/img/logo.svg" class="h-(--spacing-icon-lg)" alt="Hack4Krak logo">
    </template>

    <UNavigationMenu v-bind="navigationMenuProperties" />

    <template #right>
      <div class="flex items-center gap-3 lg:gap-4">
        <template v-if="registrationOpen && !user">
          <NuxtLink
            to="/register"
            class="hidden lg:inline-flex items-center gap-2 text-md font-semibold text-primary hover:text-primary/85 transition-colors"
            aria-label="Zapisz drużynę na Hack4Krak CTF"
          >
            <UIcon name="pixelarticons:users" class="size-4.5" />
            <span>Zapisy otwarte</span>
          </NuxtLink>

          <div class="h-5 w-px bg-white/25" />
        </template>

        <template v-if="user">
          <NuxtLink
            to="/panel/profile"
            class="flex items-center gap-2 cursor-pointer font-semibold text-md lg:hidden"
            aria-label="Profil użytkownika"
          >
            <UIcon name="pixelarticons:user" class="icon-md" />
          </NuxtLink>

          <UDropdownMenu
            :items="userMenuItems"
            :content="{ align: 'end', sideOffset: 8 }"
            :ui="{ content: 'w-48', item: 'cursor-pointer' }"
          >
            <button class="hidden lg:flex items-center gap-2 cursor-pointer font-semibold text-md" aria-label="Menu użytkownika">
              <UIcon name="pixelarticons:user" class="icon-md" />
              <span class="hidden lg:inline">{{ user.username }}</span>
            </button>
          </UDropdownMenu>
        </template>

        <NuxtLink v-else to="/login" class="text-md font-semibold flex grow-0 items-center" aria-label="Zaloguj się">
          <UIcon name="pixelarticons:login" class="icon-md lg:hidden" />
          <span class="hidden lg:inline">Zaloguj się</span>
        </NuxtLink>
      </div>
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
