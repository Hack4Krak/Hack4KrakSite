<script setup lang="ts">
import { DISCORD_URL } from '~~/content/landing-socials'
</script>

<template>
  <ElevatedButton
    :to="DISCORD_URL"
    target="_blank"
    background="#5865f2"
    rel="noopener noreferrer"
    :ui="{
      inner: 'border-[#5865f2] bg-[#5865f2] text-white flex items-center',
      content: 'items-center gap-4',
      label: '-translate-y-px',
      description: 'text-white/75 normal-case tracking-normal text-xs',
    }"
  >
    <template #leading>
      <span class="grid px-1 place-items-center text-[#5865f2]">
        <UIcon name="ic:baseline-discord" class="size-7" />
      </span>
    </template>

    Dołącz na Discorda

    <template #trailing>
      <UIcon name="pixelarticons:arrow-right" class="size-4" />
    </template>
  </ElevatedButton>
</template>
