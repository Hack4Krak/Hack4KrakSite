<script setup lang="ts">
interface BringItem {
  icon: string
  title: string
  description: string
  required?: boolean
  href?: string
  actionLabel?: string
  links?: { href: string, label: string }[]
}

defineProps<{
  items: BringItem[]
  title?: string
}>()
</script>

<template>
  <PanelCard body>
    <PanelSectionTitle class="mb-5">
      {{ title ?? 'Co zabrać ze sobą' }}
    </PanelSectionTitle>

    <ul class="grid md:grid-cols-2 gap-x-8 gap-y-5">
      <li
        v-for="item in items"
        :key="item.title"
        class="grid grid-cols-[auto_1fr] gap-4 items-start"
      >
        <span
          class="font-pixelify text-2xl leading-none tabular-nums mt-0.5"
          :class="item.required !== false ? 'text-primary' : 'text-muted/70'"
        >
          <UIcon
            :name="item.icon"
            class="size-6"
          />
        </span>

        <div>
          <p class="font-bold leading-tight flex items-center gap-2 flex-wrap">
            {{ item.title }}
          </p>
          <p class="text-sm text-muted mt-1 leading-relaxed">
            {{ item.description }}
          </p>
          <PanelActionButton
            v-if="item.href"
            :href="item.href"
            download
            icon="pixelarticons:download"
            class="mt-2"
          >
            {{ item.actionLabel ?? 'Pobierz dokument' }}
          </PanelActionButton>
          <div v-if="item.links?.length" class="mt-2 flex flex-wrap gap-x-4 gap-y-2">
            <PanelActionButton
              v-for="link in item.links"
              :key="link.href"
              :href="link.href"
              download
              icon="pixelarticons:download"
            >
              {{ link.label }}
            </PanelActionButton>
          </div>
        </div>
      </li>
    </ul>
  </PanelCard>
</template>
