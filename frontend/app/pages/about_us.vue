<script setup lang="ts">
import { aboutUsContent } from '~~/content/about-us-content'
import { aboutUsTimeline } from '~~/content/about-us-timeline'

useSeoMeta({
  title: aboutUsContent.meta.title,
  description: aboutUsContent.hero.description,
})
</script>

<template>
  <div class="flex flex-col md:gap-20 gap-14 pt-12">
    <section
      id="hero"
      class="lg:h-80 md:h-45 relative flex flex-col justify-center items-center w-full overflow-hidden
             text-center sm:items-center sm:px-0"
    >
      <UContainer class="flex flex-col justify-center items-center">
        <h1 class="text-lg font-semibold">
          {{ aboutUsContent.hero.title }}
        </h1>
        <p class="max-w-105">
          {{ aboutUsContent.hero.description }}
        </p>
      </UContainer>
      <p
        class="hidden md:block text-[7rem] lg:text-[11rem] xl:text-[14rem] font-semibold absolute -z-10 text-primary
               stroked-text-full-screen w-screen md:w-auto text-wrap"
      >
        {{ aboutUsContent.hero.backgroundText }}
      </p>
    </section>
    <section
      id="timeline"
      class="relative flex justify-center items-center"
    >
      <div class="flex md:flex-row flex-wrap flex-col gap-8 items-center justify-center">
        <EventCard v-for="card in aboutUsTimeline" v-bind="card" :key="card.title" />
      </div>
      <img
        class="absolute h-75 left-0 md:top-auto top-50 -z-20" src="assets/img/block_accent_vector.svg"
        alt="block accent vector"
        height="300px"
      >
      <img
        class="absolute h-75 right-0 md:top-auto top-150 -z-20 rotate-180" src="assets/img/block_accent_vector.svg"
        alt="block accent vector"
        height="300px"
      >
    </section>
    <section id="mission">
      <UContainer class="flex lg:flex-row flex-col-reverse mx-auto gap-12">
        <div class="flex-1 flex flex-col gap-4 sm:px-0">
          <h3 class="text-lg font-semibold">
            {{ aboutUsContent.mission.title }}
          </h3>
          <p>
            {{ aboutUsContent.mission.description }}
          </p>
          <div class="flex flex-wrap gap-4">
            <UBadge v-for="label in aboutUsContent.mission.labels" :key="label">
              {{ label }}
            </UBadge>
          </div>
          <p>
            {{ aboutUsContent.mission.additionalText }}
          </p>
        </div>
        <div class="flex-1 justify-center items-center hidden sm:flex">
          <NuxtImg class="w-full px-6 sm:px-0" src="/img/flag_n_clouds.webp" />
        </div>
      </UContainer>
    </section>
    <section id="mini-gallery">
      <UContainer class="grid lg:grid-cols-4 lg:grid-rows-[auto, auto] grid-cols-3 gap-4">
        <NuxtImg
          v-for="image in aboutUsContent.gallery.smallGalleryImages"
          :key="image.split('/').pop()" height="130"
          width="300" class="hidden md:block size-full px-6 sm:px-0 object-cover border-2 border-muted
          [&:nth-child(3)]:!border-primary"
          :src="image"
        />
        <div class="lg:row-span-2 lg:col-span-1 col-span-3 flex flex-col gap-2">
          <h3 class="text-lg font-semibold">
            {{ aboutUsContent.gallery.title }}
          </h3>
          <p v-for="paragraph in aboutUsContent.gallery.description" :key="paragraph">
            {{ paragraph }}
          </p>
        </div>
        <NuxtImg
          width="1000" height="300"
          :modifiers="{
            crop: '850_132_3681_950',
            fit: 'contain',
          }"
          class="col-span-3 w-full object-cover border-2 border-muted"
          :src="aboutUsContent.gallery.mainGalleryImage"
        />
      </UContainer>
    </section>
    <Footer />
  </div>
</template>

<style scoped>
.stroked-text-full-screen {
  -webkit-text-stroke-color: transparent;
  -webkit-text-fill-color: var(--ui-bg);
  user-select: none;
  background-color: --alpha(var(--color-primary) / 20%);
  background-clip: text;
  -webkit-text-stroke-width: 0.02em;
}
</style>
