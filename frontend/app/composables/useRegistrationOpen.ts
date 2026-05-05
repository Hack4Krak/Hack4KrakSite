import type { Ref } from 'vue'

interface RegistrationDates {
  start_date: string
  end_date: string
}

export default function useRegistrationOpen(registrationInformation: Ref<RegistrationDates | null | undefined>) {
  return computed(() => {
    const registration = registrationInformation.value
    if (!registration)
      return false

    const now = Date.now()
    return now >= new Date(registration.start_date).getTime()
      && now <= new Date(registration.end_date).getTime()
  })
}
