<script setup lang="ts">
/**
 * Hack4Krak branded OG image
 * 1200×630 (satori renders at 1200×630 by default)
 */

const props = withDefaults(defineProps<{
  title?: string
  description?: string
  siteName?: string
}>(), {
  title: 'Hack4Krak CTF',
  description: 'Największy w Małopolsce konkurs CTF dla uczniów szkół ponadpodstawowych!',
  siteName: 'hack4krak.pl',
})

const GOLD = '#d08700'
const GOLD_LIGHT = '#efb100'
const BG = '#0f0f0f'
const SURFACE = '#1a1a1a'
const MUTED = '#404040'
</script>

<template>
  <div
    :style="{
      width: '1200px',
      height: '630px',
      background: BG,
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
      overflow: 'hidden',
      fontFamily: '\'Inter\', sans-serif',
    }"
  >
    <!-- Grid pattern background -->
    <div
      :style="{
        position: 'absolute',
        inset: 0,
        backgroundImage: `linear-gradient(${MUTED}22 1px, transparent 1px), linear-gradient(90deg, ${MUTED}22 1px, transparent 1px)`,
        backgroundSize: '40px 40px',
      }"
    />

    <!-- Top-left corner accent lines -->
    <div :style="{ position: 'absolute', top: 0, left: 0, width: '300px', height: '300px', display: 'flex' }">
      <div :style="{ position: 'absolute', top: '40px', left: 0, width: '100%', height: '2px', background: `${GOLD}44` }" />
      <div :style="{ position: 'absolute', top: 0, left: '40px', width: '2px', height: '100%', background: `${GOLD}44` }" />
    </div>

    <!-- Bottom-right corner accent -->
    <div :style="{ position: 'absolute', bottom: 0, right: 0, width: '300px', height: '300px', display: 'flex' }">
      <div :style="{ position: 'absolute', bottom: '40px', right: 0, width: '100%', height: '2px', background: `${GOLD}44` }" />
      <div :style="{ position: 'absolute', bottom: 0, right: '40px', width: '2px', height: '100%', background: `${GOLD}44` }" />
    </div>

    <!-- Gold radial glow left -->
    <div
      :style="{
        position: 'absolute',
        top: '-200px',
        left: '-100px',
        width: '600px',
        height: '600px',
        background: `radial-gradient(circle, ${GOLD}28 0%, transparent 65%)`,
        borderRadius: '50%',
      }"
    />

    <!-- Gold radial glow right -->
    <div
      :style="{
        position: 'absolute',
        bottom: '-150px',
        right: '-100px',
        width: '500px',
        height: '500px',
        background: `radial-gradient(circle, ${GOLD}18 0%, transparent 60%)`,
        borderRadius: '50%',
      }"
    />

    <!-- Top gold bar -->
    <div :style="{ position: 'absolute', top: 0, left: 0, right: 0, height: '4px', background: `linear-gradient(90deg, ${GOLD}, ${GOLD_LIGHT}, ${GOLD})` }" />

    <!-- Main content -->
    <div
      :style="{
        position: 'relative',
        flex: 1,
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        padding: '60px 80px',
        gap: '24px',
      }"
    >
      <!-- CTF badge -->
      <div
        :style="{
          display: 'flex',
          alignItems: 'center',
          gap: '12px',
        }"
      >
        <div
          :style="{
            background: GOLD,
            color: '#000',
            fontSize: '12px',
            fontWeight: '800',
            letterSpacing: '0.25em',
            textTransform: 'uppercase',
            padding: '4px 14px',
            display: 'flex',
          }"
        >
          CTF
        </div>
        <div
          :style="{
            background: `${SURFACE}cc`,
            border: `1px solid ${MUTED}`,
            color: '#888',
            fontSize: '12px',
            fontWeight: '600',
            letterSpacing: '0.15em',
            textTransform: 'uppercase',
            padding: '4px 14px',
            display: 'flex',
          }"
        >
          18–19 KWIETNIA 2026 · KRAKÓW
        </div>
      </div>

      <!-- Title -->
      <div
        :style="{
          fontSize: props.title === 'Hack4Krak CTF' ? '88px' : '64px',
          fontWeight: '900',
          color: '#ffffff',
          lineHeight: '1.0',
          letterSpacing: '-0.02em',
          maxWidth: '780px',
          display: 'flex',
        }"
      >
        {{ props.title }}
      </div>

      <!-- Description -->
      <div
        :style="{
          fontSize: '26px',
          color: '#aaaaaa',
          maxWidth: '700px',
          lineHeight: '1.4',
          fontWeight: '400',
          display: 'flex',
        }"
      >
        {{ props.description }}
      </div>
    </div>

    <!-- Bottom bar -->
    <div
      :style="{
        position: 'absolute',
        bottom: 0,
        left: 0,
        right: 0,
        height: '64px',
        background: `${SURFACE}ee`,
        borderTop: `1px solid ${MUTED}66`,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '0 80px',
      }"
    >
      <!-- Logo area -->
      <div
        :style="{
          display: 'flex',
          alignItems: 'center',
          gap: '14px',
        }"
      >
        <!-- Shield icon approximation via pure CSS box -->
        <div
          :style="{
            width: '28px',
            height: '32px',
            background: GOLD,
            clipPath: 'polygon(50% 0%, 100% 15%, 100% 60%, 50% 100%, 0% 60%, 0% 15%)',
            display: 'flex',
          }"
        />
        <div
          :style="{
            fontSize: '22px',
            fontWeight: '800',
            color: '#ffffff',
            letterSpacing: '-0.01em',
            display: 'flex',
          }"
        >
          Hack4Krak
        </div>
      </div>

      <!-- Right side: URL + tagline -->
      <div
        :style="{
          display: 'flex',
          alignItems: 'center',
          gap: '20px',
        }"
      >
        <div
          :style="{
            width: '1px',
            height: '24px',
            background: MUTED,
            display: 'flex',
          }"
        />
        <div
          :style="{
            fontSize: '15px',
            color: GOLD,
            fontWeight: '600',
            letterSpacing: '0.05em',
            display: 'flex',
          }"
        >
          {{ props.siteName }}
        </div>
      </div>
    </div>

    <!-- Bottom gold bar -->
    <div :style="{ position: 'absolute', bottom: 0, left: 0, right: 0, height: '3px', background: `linear-gradient(90deg, transparent, ${GOLD}, transparent)` }" />
  </div>
</template>
