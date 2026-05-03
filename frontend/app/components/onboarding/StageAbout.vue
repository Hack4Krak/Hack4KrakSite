<script setup lang="ts">
import type { SchemaCtfExperience, SchemaSchoolGrade } from '#open-fetch-schemas/api'

const emit = defineEmits<{ submit: [] }>()

interface SelectOption<TValue extends string> {
  value: TValue
  label: string
}

type CtfLevelOption = SelectOption<SchemaCtfExperience> & {
  hint: string
}

const organization = defineModel<string>('organization', { required: true })
const location = defineModel<string>('location', { required: true })
const ctfExperience = defineModel<SchemaCtfExperience>('ctfExperience', { required: true })
const schoolGrade = defineModel<SchemaSchoolGrade>('schoolGrade', { required: true })

const SCHOOL_GRADE_OPTIONS: SelectOption<SchemaSchoolGrade>[] = [
  { value: 'NotStudying', label: 'Nie uczę się / inne' },
  { value: 'PrimarySchool', label: 'Szkoła podstawowa' },
  { value: 'Class1', label: '1. klasa szkoły średniej' },
  { value: 'Class2', label: '2. klasa szkoły średniej' },
  { value: 'Class3', label: '3. klasa szkoły średniej' },
  { value: 'Class4', label: '4. klasa szkoły średniej' },
  { value: 'Class5', label: '5. klasa szkoły średniej (technikum)' },
  { value: 'University', label: 'Studia' },
]

const CTF_LEVELS: CtfLevelOption[] = [
  {
    value: 'Never',
    label: 'Pierwszy raz słyszę o CTF',
    hint: 'Spokojnie, każdy gdzieś zaczyna. Pomożemy Ci wejść w temat.',
  },
  {
    value: 'Beginner',
    label: 'Brałem/am udział w 1–2 zawodach',
    hint: 'Dobry start. Czas na kolejne flagi.',
  },
  {
    value: 'Intermediate',
    label: '3–5 zawodów na koncie',
    hint: 'Solidne podstawy.',
  },
  {
    value: 'Advanced',
    label: '6–10 zawodów',
    hint: 'Powinniśmy się bać, czy nie zhackujesz nam strony?.',
  },
  {
    value: 'Expert',
    label: 'Powyżej 10 - robię to na poważnie',
    hint: 'To nie Ty czytasz hexdumpa - hexdump układa się pod Ciebie.',
  },
]

const sliderIndex = computed({
  get: () => Math.max(0, CTF_LEVELS.findIndex(l => l.value === ctfExperience.value)),
  set: (val: number) => {
    ctfExperience.value = CTF_LEVELS[val]?.value ?? 'Never'
  },
})

const ctfLabel = computed(() => CTF_LEVELS[sliderIndex.value]?.label ?? '')
const ctfHint = computed(() => CTF_LEVELS[sliderIndex.value]?.hint ?? '')
</script>

<template>
  <section class="space-y-6">
    <div>
      <h2 class="mb-1 text-lg font-semibold sm:text-xl">
        Z jakiej organizacji jesteś?
      </h2>
      <p class="text-sm text-muted mb-3">
        Może to być szkoła, uczelnia, firma albo inna społeczność.
      </p>
      <UInput
        v-model="organization"
        size="lg"
        placeholder="np. 31 LO Kraków, Hack4Krak, Zerya"
        icon="lucide:building-2"
        :ui="{ base: 'text-sm sm:text-base' }"
        autofocus
        @keyup.enter="emit('submit')"
      />
    </div>

    <div>
      <h2 class="text-base font-semibold sm:text-lg">
        Z jakiej miejscowości do nas zaglądasz?
      </h2>
      <UInput
        v-model="location"
        size="lg"
        placeholder="np. Kraków"
        icon="lucide:map-pin"
        :ui="{ base: 'text-sm sm:text-base' }"
        @keyup.enter="emit('submit')"
      />
    </div>

    <div>
      <h2 class="mb-1 text-base font-semibold sm:text-lg">
        Na jakim etapie nauki jesteś?
      </h2>
      <USelect
        v-model="schoolGrade"
        :items="SCHOOL_GRADE_OPTIONS"
        size="lg"
        class="w-full"
      />
    </div>

    <div>
      <h2 class="mb-1 text-base font-semibold sm:text-lg">
        Twoje doświadczenie z CTF
      </h2>
      <p class="text-sm text-muted mb-5">
        Przesuń, żeby pokazać, ile zawodów masz już za sobą.
      </p>
      <div class="space-y-4 bg-elevated/40 p-3 sm:p-4">
        <USlider
          v-model="sliderIndex"
          :min="0"
          :max="CTF_LEVELS.length - 1"
          :step="1"
          :ui="{
            root: 'h-4',
            track: 'bg-default h-2 rounded-none',
            range: 'bg-primary rounded-none',
            thumb: 'block w-4 h-4 bg-primary border-2 border-default rounded-none ring-0 shadow-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 focus-visible:ring-offset-default transition-transform hover:scale-110 active:scale-95 cursor-pointer',
          }"
        />
        <div class="flex items-start gap-3 min-h-20 sm:min-h-16">
          <span class="font-pixelify text-primary text-sm sm:text-base shrink-0 leading-none lowercase tracking-wide">
            lvl {{ sliderIndex + 1 }}
          </span>
          <div class="min-w-0 flex-1">
            <p class="font-semibold leading-tight">
              {{ ctfLabel }}
            </p>
            <p class="text-sm text-muted leading-snug mt-0.5">
              {{ ctfHint }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
