<script setup lang="ts">
interface TimelineEntry {
  time: string
  title: string
  at: Date
}

const { data: eventInfo } = await useApi('/event/info')

const schedule = computed<TimelineEntry[]>(() =>
  [...(eventInfo.value?.stages ?? [])]
    .sort((a, b) => new Date(a.start_date).getTime() - new Date(b.start_date).getTime())
    .map(stage => ({
      time: new Date(stage.start_date).toLocaleTimeString('pl-PL', { hour: '2-digit', minute: '2-digit' }),
      title: stage.name,
      at: new Date(stage.start_date),
    })),
)

const now = useNow({ interval: 30_000 })

const enriched = computed(() =>
  schedule.value.map((entry, i) => {
    const next = schedule.value[i + 1]
    const ended = entry.at < now.value && (!next || next.at < now.value)
    const active = entry.at <= now.value && (!next || next.at > now.value)
    return { ...entry, active, ended }
  }),
)

const nextEntry = computed(() => enriched.value.find(e => !e.ended && !e.active))
</script>

<template>
  <section class="flex h-full min-h-0 flex-col border-2 border-surface-muted bg-default p-5 shadow-[inset_0_0_0_1px_rgb(250_250_250/0.025)] lg:p-6">
    <div class="mb-4 flex shrink-0 items-center justify-between gap-3">
      <PanelSectionTitle>
        Harmonogram
      </PanelSectionTitle>
      <span v-if="nextEntry" class="font-pixelify text-[11px] uppercase tracking-wider text-muted">
        Następnie: <span class="text-primary">{{ nextEntry.time }}</span>
      </span>
    </div>

    <ol class="min-h-0 flex-1 space-y-3 overflow-y-auto pr-1">
      <li
        v-for="entry in enriched"
        :key="entry.time"
        class="grid grid-cols-[auto_1fr_auto] items-center gap-3"
      >
        <span
          class="size-2"
          :class="[
            entry.active ? 'bg-primary' : entry.ended ? 'bg-surface-muted' : 'bg-muted/60',
          ]"
        />
        <p
          class="truncate text-sm"
          :class="[
            entry.active ? 'font-bold text-primary' : entry.ended ? 'text-muted line-through' : 'text-default',
          ]"
        >
          {{ entry.title }}
        </p>
        <span
          class="font-pixelify text-xs tabular-nums"
          :class="entry.active ? 'text-primary' : 'text-muted'"
        >
          {{ entry.time }}
        </span>
      </li>
    </ol>

    <NuxtLink
      to="/panel/event"
      class="mt-4 flex shrink-0 items-center justify-between border-t-2 border-surface-muted pt-3 font-pixelify text-xs uppercase tracking-wider text-muted transition-colors hover:text-primary"
    >
      Cały harmonogram
      <UIcon name="pixelarticons:arrow-right" class="size-4" />
    </NuxtLink>
  </section>
</template>
