<script setup lang="ts">
import { FOOTER_SOCIALS } from '~~/content/footer-socials'

const { commitHash, branchName } = useGitInfo()
</script>

<template>
  <footer class="w-full text-muted">
    <UContainer>
      <USeparator />
      <div class="flex flex-col-reverse gap-4 lg:flex-row justify-between w-full py-8 px-12 lg:mb-8 text-xs lg:text-sm">
        <!-- Left section -->
        <div class="flex flex-col items-center lg:items-baseline justify-between gap-2 mb-5 lg:mb-0">
          <!-- Logo section -->
          <LogoWithText size="sm" />

          <!-- Copyright -->
          <p class="text-center lg:text-left">
            © {{ new Date().getFullYear() }} Wszelkie prawa zastrzeżone.
          </p>

          <!-- Open source link -->
          <div class="hidden lg:block">
            Jesteśmy
            <NuxtLink to="https://github.com/Hack4Krak" class="underline">
              <span>Open Source</span>
            </NuxtLink>!
          </div>
          <div class="hidden lg:block">
            <span class="text-xs text-dimmed">{{ branchName }}@{{ commitHash }}</span>
          </div>
        </div>

        <!-- Right section - Social links grid -->
        <div class="text-center lg:w-1/2 lg:grid grid-rows-2 grid-flow-col-dense place-content-between">
          <NuxtLink
            v-for="{ label, icon, to } in FOOTER_SOCIALS"
            :key="label"
            :to="to"
            class="group lg:flex items-center lg:flex-row-reverse gap-2 hover:text-default transition-all duration-150 ease-in-out"
          >
            <UIcon
              v-if="icon"
              class="hidden lg:block"
              :name="icon"
              size="15"
            />
            <span class="font-medium lg:font-normal">
              {{ label }}
            </span>
            <span class="group-[&:last-child]:hidden lg:hidden"> | </span>
          </NuxtLink>
        </div>
      </div>
    </UContainer>
  </footer>
</template>
