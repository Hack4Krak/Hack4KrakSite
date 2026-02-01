<script setup lang="ts">
import { FOOTER_SOCIALS } from '~~/content/footer-socials'

const runtimeConfig = useRuntimeConfig().public
</script>

<template>
  <USeparator />
  <UFooter class="w-full text-muted py-8 lg:px-12 lg:mb-8 text-xs lg:text-base">
    <template #left>
      <div class="flex flex-col items-center lg:items-start justify-between text-xs lg:text-base">
        <img src="~/assets/img/logo.svg" class="h-(--spacing-icon-lg) mb-4 lg:mb-4" alt="Hack4Krak logo">

        <p class="text-center lg:text-left">
          © {{ new Date().getFullYear() }} Wszelkie prawa zastrzeżone.
        </p>

        <div class="hidden lg:block">
          Jesteśmy
          <NuxtLink to="https://github.com/Hack4Krak" class="underline" target="_blank">
            <span>Open Source</span>
          </NuxtLink>!
        </div>
        <div v-if="runtimeConfig.gitCommit" class="hidden lg:block">
          <span aria-hidden="true" class="text-xs text-dimmed">{{ runtimeConfig.gitBranch }}@{{ runtimeConfig.gitCommit }}</span>
        </div>
      </div>
    </template>

    <template #right>
      <div class="text-center w-full lg:grid grid-rows-2 grid-flow-col-dense place-content-center lg:place-content-between">
        <ULink
          v-for="{ label, icon, to, target } in FOOTER_SOCIALS"
          :key="label"
          :to="to"
          :target="target"
          class="group lg:flex items-center lg:flex-row-reverse gap-2 hover:text-default transition-all duration-150 ease-in-out"
        >
          <UIcon
            v-if="icon"
            class="hidden lg:block"
            :name="icon"
            size="15"
          />
          <span class="lg:font-normal p-3 leading-6">
            {{ label }}
          </span>
          <span class="font-thin group-[&:last-child]:hidden lg:hidden"> | </span>
        </ULink>
      </div>
    </template>
  </UFooter>
</template>
