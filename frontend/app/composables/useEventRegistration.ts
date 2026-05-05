import { useLocalStorage } from '@vueuse/core'

/**
 * Local draft for the multi-step event registration UI.
 * Backend-backed pieces like team membership are handled separately.
 */
export interface EventRegistrationDraft {
  agreements: {
    rules: boolean
    rodo: boolean
    parental: boolean
    bringConsents: boolean
  }
  personal: {
    fullName: string
    school: string
    birthYear: string
    isUnderage: boolean
    foodPreference: 'standard' | 'vegetarian' | ''
    foodAllergies: string
    phone: string
    emergencyContactName: string
    emergencyContactPhone: string
    emergencyContactEmail: string
  }
  team: {
    confirmed: boolean
    teamName: string
  }
  optional: {
    confirmAttendance: boolean
    bringOwnLaptop: boolean
    bringDocuments: boolean
  }
  status: 'draft' | 'submitted'
  submittedAt?: string
}

const STORAGE_KEY = 'h4k-event-registration-draft-2026'

function emptyDraft(): EventRegistrationDraft {
  return {
    agreements: {
      rules: false,
      rodo: false,
      parental: false,
      bringConsents: false,
    },
    personal: {
      fullName: '',
      school: '',
      birthYear: '',
      isUnderage: true,
      foodPreference: '',
      foodAllergies: '',
      phone: '',
      emergencyContactName: '',
      emergencyContactPhone: '',
      emergencyContactEmail: '',
    },
    team: {
      confirmed: false,
      teamName: '',
    },
    optional: {
      confirmAttendance: false,
      bringOwnLaptop: false,
      bringDocuments: false,
    },
    status: 'draft',
  }
}

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return Object.prototype.toString.call(value) === '[object Object]'
}

function deepMerge<T>(defaults: T, stored: unknown): T {
  if (!isPlainObject(defaults) || !isPlainObject(stored)) {
    return stored === undefined ? defaults : stored as T
  }

  const merged: Record<string, unknown> = { ...defaults }

  for (const key of Object.keys(stored)) {
    merged[key] = deepMerge(
      (defaults as Record<string, unknown>)[key],
      stored[key],
    )
  }

  return merged as T
}

export function useEventRegistrationDraft() {
  const draftReady = ref(false)

  const draft = useLocalStorage<EventRegistrationDraft>(
    STORAGE_KEY,
    emptyDraft(),
    {
      initOnMounted: true,
      mergeDefaults: (storedValue, defaults) => {
        return deepMerge(defaults, storedValue)
      },
    },
  )

  onMounted(() => {
    draftReady.value = true
  })

  function resetDraft() {
    draft.value = emptyDraft()
  }

  function markDraftSubmitted() {
    draft.value.status = 'submitted'
    draft.value.submittedAt = new Date().toISOString()
  }

  return {
    draft,
    draftReady,
    resetDraft,
    markDraftSubmitted,
  }
}

export default function useEventRegistration() {
  const {
    draft,
    draftReady,
    resetDraft,
    markDraftSubmitted,
  } = useEventRegistrationDraft()

  const {
    data: team,
    pending: teamPending,
    error: teamError,
    refresh: refreshTeam,
  } = useAuth('/teams/membership/my_team', {
    onResponseError: () => {
      throw new Error('Response error')
    },
  })

  const {
    data: participation,
    pending: participationPending,
    error: participationError,
    refresh: refreshParticipation,
    status: participationStatus,
  } = useAuth('/event/participate', {
    onResponseError: () => {
      throw new Error('Response error')
    },
  })

  const submitting = ref(false)
  const submitError = ref<unknown>(null)

  const hasTeam = computed(() => Boolean(team.value))
  const isRegistered = computed(() => Boolean(participationStatus.value === 'success'))

  const pending = computed(() => {
    return teamPending.value || participationPending.value || submitting.value
  })

  async function submitRegistration() {
    submitError.value = null
    submitting.value = true

    const { $auth } = useNuxtApp()

    try {
      const { personal } = draft.value
      await $auth('/event/participate', {
        method: 'POST',
        body: {
          full_name: personal.fullName,
          school: personal.school,
          birth_year: personal.birthYear,
          phone: personal.phone,
          is_underage: personal.isUnderage,
          emergency_contact_name: personal.emergencyContactName.trim() || null,
          emergency_contact_phone: personal.emergencyContactPhone.trim() || null,
          emergency_contact_email: personal.emergencyContactEmail.trim() || null,
          food_preference: personal.foodPreference === 'vegetarian' ? 'vegetarian' : 'standard',
          food_allergies: personal.foodAllergies.trim() || null,
        },
      })

      markDraftSubmitted()
      await refreshParticipation()
    } catch (error) {
      submitError.value = error
      throw error
    } finally {
      submitting.value = false
    }
  }

  function reset() {
    resetDraft()
  }

  return {
    draft,
    draftReady,

    team,
    participation,

    hasTeam,
    isRegistered,

    pending,
    teamPending,
    participationPending,
    submitting,

    teamError,
    participationError,
    submitError,

    refreshTeam,
    refreshParticipation,

    reset,
    submitRegistration,
  }
}
