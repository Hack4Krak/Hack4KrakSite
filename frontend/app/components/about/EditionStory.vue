<script setup lang="ts">
import type { AboutUsEdition } from '~~/content/about-us-content'

const props = defineProps<{
  editions: AboutUsEdition[]
}>()

const active = ref(0)
const chapterElements = useTemplateRef<HTMLElement[]>('chapter')

let observer: IntersectionObserver | undefined

// The chapter crossing the middle of the viewport becomes the active edition
onMounted(() => {
  observer = new IntersectionObserver((entries) => {
    for (const entry of entries) {
      if (entry.isIntersecting)
        active.value = Number((entry.target as HTMLElement).dataset.index)
    }
  }, { rootMargin: '-45% 0px -45% 0px' })

  chapterElements.value?.forEach(element => observer!.observe(element))
})

onBeforeUnmount(() => observer?.disconnect())

function scrollToChapter(index: number) {
  chapterElements.value?.[index]?.scrollIntoView({ block: 'start' })
}

interface PhotoSlot {
  class: string
  tilt: number
  drift: number
}

interface ChapterLayout {
  text: string
  slots: PhotoSlot[]
}

// Hand-placed spots on a 12-column grid. Next to the text sits a lead photo with the edition's portrait photo
// pinned over its corner, the rest go in rows below. Chapters alternate sides so the page does not repeat itself.
// A photo under the text needs a top margin larger than its drift, or it slides over the stats while scrolling
const LAYOUTS: Record<'left' | 'right' | 'finale', ChapterLayout> = {
  left: {
    text: 'lg:col-start-1',
    slots: [
      { class: 'lg:col-start-8 lg:col-span-5 lg:row-start-1 lg:mt-4', tilt: 3, drift: 40 },
      { class: 'lg:col-start-8 lg:col-span-3 lg:row-start-2 lg:-mt-10 lg:-ml-4 lg:z-10', tilt: -4, drift: 60 },
      { class: 'lg:col-start-1 lg:col-span-4 lg:row-start-3 lg:mt-20', tilt: -2, drift: 30 },
      { class: 'lg:col-start-5 lg:col-span-4 lg:row-start-3 lg:mt-28 lg:ml-4', tilt: 2, drift: 50 },
      { class: 'lg:col-start-9 lg:col-span-4 lg:row-start-3 lg:mt-16 lg:ml-4', tilt: -3, drift: 40 },
    ],
  },
  right: {
    text: 'lg:col-start-6',
    slots: [
      { class: 'lg:col-start-1 lg:col-span-5 lg:row-start-1 lg:mt-4', tilt: -3, drift: 40 },
      { class: 'lg:col-start-3 lg:col-span-3 lg:row-start-2 lg:-mt-10 lg:ml-4 lg:z-10', tilt: 4, drift: 60 },
      { class: 'lg:col-start-1 lg:col-span-4 lg:row-start-3 lg:mt-16', tilt: 2, drift: 40 },
      { class: 'lg:col-start-5 lg:col-span-4 lg:row-start-3 lg:mt-28 lg:ml-4', tilt: -2, drift: 50 },
      { class: 'lg:col-start-9 lg:col-span-4 lg:row-start-3 lg:mt-20 lg:ml-4', tilt: 3, drift: 30 },
    ],
  },
  finale: {
    text: 'lg:col-start-1',
    slots: [
      { class: 'lg:col-start-8 lg:col-span-5 lg:row-start-1 lg:mt-4', tilt: 2, drift: 40 },
      { class: 'lg:col-start-10 lg:col-span-3 lg:row-start-2 lg:-mt-10 lg:z-10', tilt: -3, drift: 60 },
      { class: 'lg:col-start-1 lg:col-span-4 lg:row-start-3 lg:mt-20', tilt: -3, drift: 40 },
      { class: 'lg:col-start-5 lg:col-span-4 lg:row-start-3 lg:mt-28 lg:ml-4', tilt: 1, drift: 30 },
      { class: 'lg:col-start-9 lg:col-span-4 lg:row-start-3 lg:mt-16 lg:ml-4', tilt: -2, drift: 50 },
      { class: 'lg:col-start-2 lg:col-span-5 lg:row-start-4 lg:mt-10', tilt: 3, drift: 35 },
      { class: 'lg:col-start-7 lg:col-span-5 lg:row-start-4 lg:mt-16 lg:ml-6', tilt: -2, drift: 45 },
    ],
  },
}

const lastWithPhotos = computed(() => props.editions.findLastIndex(edition => edition.photos.length > 0))

function layoutFor(index: number) {
  if (index === lastWithPhotos.value)
    return LAYOUTS.finale
  return index % 2 === 0 ? LAYOUTS.left : LAYOUTS.right
}

function textClass(edition: AboutUsEdition, index: number) {
  return edition.photos.length ? `lg:col-span-7 ${layoutFor(index).text}` : 'lg:col-span-12'
}

const current = computed(() => props.editions[active.value] ?? props.editions[0]!)
const padded = (edition: AboutUsEdition) => String(edition.number).padStart(2, '0')
</script>

<template>
  <div class="grid gap-10 lg:grid-cols-[16rem_1fr] lg:gap-20">
    <div class="hidden lg:block">
      <div class="sticky top-[calc(var(--ui-header-height)+4rem)] flex flex-col gap-10">
        <div>
          <p class="text-sm text-muted">
            Edycja
          </p>
          <p class="font-pixelify text-[8rem] leading-none font-bold text-primary tabular-nums">
            {{ padded(current) }}
          </p>
          <p class="mt-2 font-pixelify text-2xl text-default">
            {{ current.date }}
          </p>
        </div>

        <ol class="flex flex-col border-l-2 border-surface-muted" aria-label="Przejdź do edycji">
          <li v-for="(edition, index) in editions" :key="edition.number">
            <button
              type="button"
              class="-ml-0.5 w-full cursor-pointer border-l-2 py-2 pl-4 text-left transition-colors"
              :class="index === active ? 'border-primary text-default' : 'border-transparent text-muted hover:text-default'"
              :aria-current="index === active ? 'step' : undefined"
              @click="scrollToChapter(index)"
            >
              <span class="block font-pixelify">{{ edition.date }}</span>
              <span class="block text-sm">{{ edition.title }}</span>
            </button>
          </li>
        </ol>
      </div>
    </div>

    <div class="sticky top-(--ui-header-height) z-20 -mx-4 flex items-baseline gap-3 border-b-2 border-surface-muted bg-default/95 px-4 py-3 backdrop-blur-sm sm:-mx-6 sm:px-6 lg:hidden">
      <p class="font-pixelify text-3xl font-bold text-primary tabular-nums">
        {{ padded(current) }}
      </p>
      <p class="font-pixelify text-lg text-default">
        {{ current.date }}
      </p>
    </div>

    <ol class="flex flex-col">
      <li
        v-for="(edition, index) in editions"
        :id="`edycja-${edition.number}`"
        ref="chapter"
        :key="edition.number"
        :data-index="index"
        class="grid scroll-mt-[calc(var(--ui-header-height)+2rem)] grid-cols-2 content-start gap-x-4 gap-y-6 py-16 first:pt-0 lg:grid-cols-12 lg:gap-x-6 lg:gap-y-0 lg:py-20"
      >
        <div class="col-span-2 flex flex-col gap-5 lg:row-span-2 lg:row-start-1" :class="textClass(edition, index)">
          <div class="flex flex-wrap items-center gap-3">
            <p class="font-pixelify text-lg text-primary">
              Edycja {{ edition.number }}, {{ edition.date }}
            </p>
            <span v-if="edition.planned" class="inline-flex items-center gap-1.5 bg-primary px-2 py-0.5 font-pixelify text-sm text-black">
              <UIcon name="pixelarticons:clock" class="size-4" aria-hidden="true" />
              w przygotowaniu
            </span>
          </div>
          <h2 class="font-pixelify text-4xl leading-tight text-balance text-default lg:text-6xl">
            {{ edition.title }}
          </h2>
          <p v-for="paragraph in edition.paragraphs" :key="paragraph" class="text-base leading-relaxed text-default/80 lg:text-lg">
            {{ paragraph }}
          </p>

          <dl class="mt-2 grid max-w-3xl grid-cols-3 gap-4 border-y-2 border-surface-muted py-5">
            <div v-for="stat in edition.stats" :key="stat.label">
              <dt class="sr-only">
                {{ stat.label }}
              </dt>
              <dd class="font-pixelify text-2xl text-primary sm:text-4xl">
                {{ stat.value }}
              </dd>
              <dd class="mt-1 text-sm text-muted">
                {{ stat.label }}
              </dd>
            </div>
          </dl>

          <LandingV4Flags v-if="edition.flags" class="mt-2" />
        </div>

        <PixelPhoto
          v-for="(photo, photoIndex) in edition.photos.slice(0, layoutFor(index).slots.length)"
          :key="photo.src"
          :src="photo.src"
          :alt="photo.alt"
          :ratio="photo.ratio"
          :width="600"
          zoom
          class="drift self-start"
          :class="[layoutFor(index).slots[photoIndex]!.class, { 'col-span-2': photoIndex === 0 && Math.min(edition.photos.length, layoutFor(index).slots.length) % 2 === 1 }]"
          :tilt="layoutFor(index).slots[photoIndex]!.tilt"
          :style="{ '--drift': `${layoutFor(index).slots[photoIndex]!.drift}px` }"
        />
      </li>
    </ol>
  </div>
</template>

<style scoped>
/* Photos float at slightly different speeds while scrolling, where the browser supports scroll-driven animations */
@supports (animation-timeline: view()) {
  @media (prefers-reduced-motion: no-preference) {
    .drift {
      animation: drift linear both;
      animation-timeline: view();
    }
  }
}

@keyframes drift {
  from {
    translate: 0 var(--drift);
  }
  to {
    translate: 0 calc(var(--drift) * -1);
  }
}
</style>
