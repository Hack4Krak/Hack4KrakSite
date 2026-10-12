<script setup lang="ts">
import { FOOTER_CONTACTS, FOOTER_FOUNDATION_NOTE, FOOTER_LINKS, FOOTER_SOCIALS } from '~~/content/footer'
import { FOUNDATION, SOURCE_CODE_URL, TAGLINE } from '~~/content/organization'

withDefaults(defineProps<{
  separator?: boolean
}>(), {
  separator: true,
})

const runtimeConfig = useRuntimeConfig().public
const foundationHost = new URL(FOUNDATION.url).host

const COLUMNS = [
  { title: 'Kontakt', links: FOOTER_CONTACTS },
  { title: 'Informacje', links: FOOTER_LINKS },
]
</script>

<template>
  <!-- Keeps its own container, so it lines up with the page no matter where it is placed -->
  <UContainer as="footer" class="w-full text-sm">
    <USeparator v-if="separator" />

    <div class="grid grid-cols-2 gap-x-8 gap-y-12 py-14 lg:grid-cols-[1fr_auto_auto_auto] lg:gap-x-12 xl:gap-x-20">
      <div class="col-span-2 flex flex-col gap-5 sm:col-span-1">
        <NuxtLink to="/" aria-label="Hack4Krak, strona główna" class="self-start">
          <img src="~/assets/img/logo.svg" class="h-8" alt="Hack4Krak logo">
        </NuxtLink>
        <p class="max-w-xs leading-relaxed text-muted">
          {{ TAGLINE }}
        </p>
        <ul class="-ml-2 flex gap-1" aria-label="Hack4Krak w mediach społecznościowych">
          <li v-for="social in FOOTER_SOCIALS" :key="social.label">
            <ULink
              :to="social.to"
              :target="social.target"
              :aria-label="social.label"
              :title="social.label"
              class="flex size-10 items-center justify-center text-muted transition-colors hover:text-primary"
            >
              <UIcon :name="social.icon!" class="size-6" />
            </ULink>
          </li>
        </ul>
        <p class="text-xs whitespace-nowrap text-dimmed">
          © {{ new Date().getFullYear() }} Hack4Krak
          <template v-if="runtimeConfig.gitCommit">
            <span aria-hidden="true" class="mx-1.5">·</span>
            <ULink :to="`${SOURCE_CODE_URL}/commit/${runtimeConfig.gitCommit}`" target="_blank" title="Wersja strony" class="text-dimmed hover:text-muted">
              {{ runtimeConfig.gitBranch }}@{{ runtimeConfig.gitCommit }}
            </ULink>
          </template>
        </p>
      </div>

      <nav v-for="column in COLUMNS" :key="column.title" :aria-label="column.title">
        <p class="font-pixelify text-default">
          {{ column.title }}
        </p>
        <ul class="mt-4 flex flex-col gap-3 whitespace-nowrap">
          <li v-for="link in column.links" :key="link.label">
            <ULink :to="link.to" :target="link.target" class="text-muted hover:text-default">
              {{ link.label }}
            </ULink>
          </li>
        </ul>
      </nav>

      <div class="col-span-2 flex max-w-xs flex-col gap-4 sm:col-span-1">
        <p class="font-pixelify text-default">
          Oficjalny organizator
        </p>
        <a :href="FOUNDATION.url" target="_blank" rel="noopener" class="self-start">
          <img :src="FOUNDATION.logo" :alt="FOUNDATION.name" class="h-20 w-auto">
        </a>
        <p class="leading-relaxed text-muted">
          {{ FOOTER_FOUNDATION_NOTE }}
        </p>
        <ULink :to="FOUNDATION.url" target="_blank" class="inline-flex items-center gap-2 self-start text-primary hover:underline">
          {{ foundationHost }}
          <UIcon name="pixelarticons:external-link" class="size-4" />
        </ULink>
      </div>
    </div>
  </UContainer>
</template>
