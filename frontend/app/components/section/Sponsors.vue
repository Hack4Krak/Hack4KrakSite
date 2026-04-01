<script setup lang="ts">
interface Sponsor {
  name: string
  url: string
  tagline: string
  logo: string | null
  logoAlt: string | null
  placeholder: boolean
  featured?: boolean
}

const sponsors: Sponsor[] = [
  {
    name: 'CyberFolks',
    url: 'https://cyberfolks.pl',
    tagline: 'Sponsor Główny',
    logo: '/img/cyberfolks-logo.png',
    logoAlt: 'CyberFolks',
    placeholder: false,
    featured: true,
  },
  {
    name: 'Arkanet',
    url: 'https://arkanet.pl',
    tagline: 'Sponsor',
    logo: '/img/arkanet-logo-white.webp',
    logoAlt: 'Arkanet',
    placeholder: false,
    featured: true,
  },
  {
    name: 'Zerya',
    url: 'https://zerya.dev',
    tagline: 'Partner technologiczny',
    logo: '/img/zerya-logo.png',
    logoAlt: 'Zerya',
    placeholder: false,
    featured: false,
  },
  {
    name: '?',
    url: 'mailto:kontakt@hack4krak.pl',
    tagline: 'Zostań partnerem',
    logo: null,
    logoAlt: null,
    placeholder: true,
    featured: false,
  },
]

const featuredSponsors = sponsors.filter(s => s.featured)
const otherSponsors = sponsors.filter(s => !s.featured)
</script>

<template>
  <section class="w-full py-12 lg:py-20">
    <div class="text-center mb-10 lg:mb-14">
      <p class="text-xs font-bold tracking-[0.25em] uppercase text-muted mb-3">
        Partnerzy i Sponsorzy
      </p>
      <h2 class="font-pixelify text-3xl lg:text-5xl text-default">
        Wspierają nas
      </h2>
    </div>

    <!-- Featured / paying sponsors - larger cards -->
    <div class="flex flex-wrap justify-center items-stretch gap-8 lg:gap-12 mb-8">
      <a
        v-for="sponsor in featuredSponsors"
        :key="sponsor.name"
        :href="sponsor.url"
        target="_blank"
        rel="noopener noreferrer"
        class="group relative flex flex-col items-center justify-center gap-4 px-14 py-12
               border-2 border-primary transition-all duration-300 cursor-pointer 
               w-[280px] lg:w-[320px] bg-primary/5 hover:bg-primary/10"
        :aria-label="`Sponsor: ${sponsor.name}`"
      >
        <!-- "Sponsor" badge -->
        <span class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2
                     bg-primary text-default text-[10px] font-bold tracking-widest uppercase
                     px-4 py-1 font-pixelify whitespace-nowrap">
          {{ sponsor.tagline }}
        </span>
        
        <!-- Logo -->
        <div class="h-24 flex items-center justify-center w-full">
          <img
            v-if="sponsor.logo"
            :src="sponsor.logo"
            :alt="sponsor.logoAlt!"
            class="max-h-24 max-w-[240px] w-auto h-auto object-contain transition-all duration-300"
          >
        </div>
      </a>
    </div>

    <!-- Regular partners + placeholder -->
    <div class="flex flex-wrap justify-center items-stretch gap-6 lg:gap-8">
      <a
        v-for="sponsor in otherSponsors"
        :key="sponsor.name"
        :href="sponsor.url"
        :target="sponsor.placeholder ? '_self' : '_blank'"
        rel="noopener noreferrer"
        class="group flex flex-col items-center justify-center gap-3 px-8 py-8
               border-2 transition-all duration-300 cursor-pointer w-[240px]"
        :class="sponsor.placeholder
          ? 'border-dashed border-surface-muted hover:border-primary opacity-50 hover:opacity-100'
          : 'border-surface-muted hover:border-primary'"
        :aria-label="sponsor.placeholder ? 'Zostań sponsorem' : `Partner: ${sponsor.name}`"
      >
        <div
          class="h-20 flex items-center justify-center transition-colors duration-300"
          :class="sponsor.placeholder ? 'text-muted group-hover:text-primary' : 'text-default'"
        >
          <img
            v-if="sponsor.logo"
            :src="sponsor.logo"
            :alt="sponsor.logoAlt ?? sponsor.name"
            class="max-h-20 max-w-[180px] w-auto h-auto object-contain transition-all duration-300"
          >
          <span
            v-else
            class="font-pixelify tracking-widest font-bold text-3xl"
          >
            {{ sponsor.name }}
          </span>
        </div>
        <span class="text-xs text-muted group-hover:text-default transition-colors duration-300 text-center">
          {{ sponsor.tagline }}
        </span>
      </a>
    </div>

    <p class="text-center text-xs text-muted mt-10">
      Zainteresowany partnerstwem? Napisz do nas:
      <a href="mailto:kontakt@hack4krak.pl" class="link">kontakt@hack4krak.pl</a>
    </p>
  </section>
</template>