<script setup lang="ts">
import { aboutUsContent } from '~~/content/about-us-content'
import { FOUNDATION, SOURCE_CODE_URL } from '~~/content/organization'

useSeoMeta({
  title: aboutUsContent.meta.title,
  description: aboutUsContent.meta.description,
})

const linkClass = 'font-bold text-primary underline-offset-4 hover:underline'
</script>

<template>
  <AboutHeader
    :kicker="aboutUsContent.hero.subtitle"
    :title="aboutUsContent.hero.title"
    :description="aboutUsContent.hero.description"
    :photos="aboutUsContent.hero.photos"
  />

  <UContainer class="w-full">
    <AboutEditionStory :editions="aboutUsContent.editions" class="mt-8 lg:mt-16" />

    <section class="mt-28 grid items-center gap-12 lg:mt-36 lg:grid-cols-[1fr_1.1fr] lg:gap-20">
      <div class="flex flex-col">
        <PixelPhoto src="/img/our-team.webp" alt="Zespół organizatorów Hack4Krak" :tilt="-2" ratio="2/1" :width="1000" zoom />
        <!-- The foundation's logo pinned over the photo's corner like a second print, in the same frame -->
        <a
          :href="FOUNDATION.url"
          target="_blank"
          rel="noopener"
          class="relative z-10 -mt-8 -mr-2 flex w-48 items-center justify-center self-end rotate-3 border-4 border-text-default bg-default px-6 py-5 shadow-[6px_6px_0_0_var(--ui-primary)] sm:-mt-12 sm:w-60 sm:border-8 sm:shadow-[8px_8px_0_0_var(--ui-primary)]"
        >
          <img :src="FOUNDATION.logo" :alt="FOUNDATION.name" class="h-12 w-auto sm:h-16">
        </a>
      </div>

      <div class="flex flex-col gap-6">
        <h2 class="font-pixelify text-4xl leading-tight text-balance text-default lg:text-6xl">
          Za każdą edycją stoją ludzie
        </h2>
        <p class="text-base leading-relaxed text-default/80 lg:text-lg">
          Przy ostatniej edycji pracowało ponad 30 wolontariuszy: autorzy zadań, osoby od logistyki, grafiki, zdjęć i opieki nad uczestnikami.
        </p>

        <p class="text-base leading-relaxed text-default/80 lg:text-lg">
          Żeby przygotowywać wydarzenia na większą skalę, założyliśmy
          <a :href="FOUNDATION.url" target="_blank" rel="noopener" :class="linkClass">Fundację Zerya</a>.
          Fundacja jest oficjalnym organizatorem konkursu, a w przyszłości także innych wydarzeń.
        </p>
        <p class="font-pixelify text-2xl text-primary">
          Dziękujemy każdemu, kto kiedykolwiek pomógł.
        </p>
        <p class="text-sm leading-relaxed text-muted">
          Kod platformy znajdziesz na <a :href="SOURCE_CODE_URL" target="_blank" rel="noopener" :class="linkClass">GitHubie</a>.
        </p>
      </div>
    </section>

    <TrustedBy class="mt-28 lg:mt-36" />
  </UContainer>

  <AboutJoinUs />

  <Footer />
</template>
