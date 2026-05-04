<script setup lang="ts">
const { data: user } = await useAuth('/account/')

const route = useRoute()

const ACCOUNT_NAVIGATION = [
  { to: '/account/events', label: 'Wydarzenia', icon: 'pixelarticons:calendar' },
  { to: '/account/settings', label: 'Ustawienia konta', icon: 'pixelarticons:sliders-2' },
] as const

function isActivePath(path: string) {
  return route.path === path
}
</script>

<template>
  <NuxtLayout name="default">
    <div class="flex-1 grid grid-cols-1 lg:grid-cols-[280px_1fr] gap-6 lg:gap-0 lg:divide-x-2 divide-surface-muted m-4 lg:m-8 border-2 border-surface-muted bg-default min-h-[calc(100vh-12rem)]">
      <aside class="flex flex-col">
        <div class="p-6 flex flex-col items-center text-center border-b-2 border-surface-muted">
          <ProfilePicture :username="user?.username" />
          <h2 class="font-pixelify text-lg leading-tight">
            {{ user?.username }}
          </h2>
          <p class="text-xs text-muted mt-3 break-all">
            {{ user?.email }}
          </p>
        </div>

        <nav class="flex-1 p-3 space-y-1">
          <NuxtLink
            v-for="item in ACCOUNT_NAVIGATION"
            :key="item.to"
            :to="item.to"
            class="w-full flex items-center gap-3 px-4 py-3 text-sm transition-all cursor-pointer relative font-medium"
            :class="isActivePath(item.to)
              ? 'bg-primary/10 text-default'
              : 'text-muted hover:text-default hover:bg-surface-muted/40'"
          >
            <span
              v-if="isActivePath(item.to)"
              class="absolute left-0 top-2 bottom-2 w-1 bg-primary"
            />
            <UIcon :name="item.icon" class="size-4 shrink-0" />
            <span>{{ item.label }}</span>
          </NuxtLink>
        </nav>

        <div class="p-3 border-t-2 border-surface-muted space-y-1">
          <NuxtLink
            to="/"
            class="w-full flex items-center gap-3 px-4 py-2.5 text-sm text-muted hover:text-default transition-colors"
          >
            <UIcon name="pixelarticons:home" class="size-4 shrink-0" />
            Strona główna
          </NuxtLink>
          <button
            type="button"
            class="w-full flex items-center gap-3 px-4 py-2.5 text-sm text-error/80 hover:text-error hover:bg-error/5 transition-colors cursor-pointer"
            @click="useSession().logout"
          >
            <UIcon name="pixelarticons:logout" class="size-4 shrink-0" />
            Wyloguj się
          </button>
        </div>
      </aside>

      <section class="p-6 lg:p-8 overflow-x-auto">
        <slot />
      </section>
    </div>
  </NuxtLayout>
</template>
