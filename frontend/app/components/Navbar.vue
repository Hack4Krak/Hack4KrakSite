<script setup lang="ts">
import { NAVBAR_ITEMS } from '~~/content/navbar'
import LANDING_CONTENT from '~~/content/landing-page.json'

const { data: isLoggedIn } = useAuth('/auth/status', {
  redirect: 'error',
  onResponseError: undefined,
})

const { event } = LANDING_CONTENT

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
      <img src="~/assets/img/logo.svg" class="h-(--spacing-icon-lg)" alt="Hack4Krak logo">
    </template>

    <UNavigationMenu v-bind="navigationMenuProperties" />

    <template #right>
      <NuxtLink
        v-if="event.registrationOpen && !isLoggedIn"
        :to="event.registrationUrl"
        class="registration-badge hidden lg:flex items-center gap-2"
        aria-label="Zapisy otwarte — Zapisz się na Hack4Krak CTF"
      >
        <span class="live-dot" aria-hidden="true" />
        <span class="text-xs font-bold tracking-[0.15em] uppercase">Zapisy otwarte</span>
        <UIcon name="pixelarticons:chevron-right" class="size-3.5 opacity-70" />
      </NuxtLink>

      <NuxtLink to="/login" class="text-md font-semibold flex grow-0 ml-5" :aria-label="isLoggedIn ? 'Otwórz panel' : 'Zaloguj się'">
        <UIcon :name="isLoggedIn ? 'pixelarticons:user' : 'pixelarticons:login'" class="icon-md lg:hidden" />

        <span class="hidden lg:inline">
          {{ isLoggedIn ? "Otwórz panel" : "Zaloguj się" }}
        </span>
      </NuxtLink>
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

<style scoped>
.registration-badge {
  padding: 0.35rem 0.75rem 0.35rem 0.6rem;
  border: 1.5px solid var(--color-primary);
  color: var(--color-primary);
  text-decoration: none;
  transition: background 0.15s, box-shadow 0.15s;
  box-shadow: 0 0 10px color-mix(in oklch, var(--color-primary) 30%, transparent);
}

.registration-badge:hover {
  background: color-mix(in oklch, var(--color-primary) 10%, transparent);
  box-shadow: 0 0 18px color-mix(in oklch, var(--color-primary) 50%, transparent);
}

.live-dot {
  display: inline-block;
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--color-primary);
  flex-shrink: 0;
  animation: pulse-dot 1.8s ease-in-out infinite;
}

@keyframes pulse-dot {
  0%, 100% {
    opacity: 1;
    box-shadow: 0 0 0 0 color-mix(in oklch, var(--color-primary) 60%, transparent);
  }
  50% {
    opacity: 0.8;
    box-shadow: 0 0 0 5px color-mix(in oklch, var(--color-primary) 0%, transparent);
  }
}
</style>
