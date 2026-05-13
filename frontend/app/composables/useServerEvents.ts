import type {
  SchemaLeaderboardUpdateEvent,
  SchemaServerEventTopic,
  SchemaTeamFlagCaptureEvent,
} from '#open-fetch-schemas/api'
import { createSharedComposable, useEventSource } from '@vueuse/core'
import { watch } from 'vue'

const ALL_TOPICS: SchemaServerEventTopic[] = ['leaderboard', 'team']

interface ServerEventPayloads {
  leaderboard_update: SchemaLeaderboardUpdateEvent
  team_flag_capture: SchemaTeamFlagCaptureEvent
}

type ServerEventName = keyof ServerEventPayloads
type ServerEventHandler<T extends ServerEventName> = (payload: ServerEventPayloads[T]) => void

type ServerEventMessage = {
  [T in ServerEventName]: {
    type: T
    payload: ServerEventPayloads[T]
  }
}[ServerEventName]

const SERVER_EVENT_NAMES: ServerEventName[] = ['leaderboard_update', 'team_flag_capture']

function buildUrl() {
  const baseURL = useRuntimeConfig().public.openFetch.api.baseURL
  const url = new URL('/events/stream', baseURL)
  for (const topic of ALL_TOPICS) {
    url.searchParams.append('topics', topic)
  }
  return url.toString()
}

const useSharedServerEvents = createSharedComposable(() => useEventSource<ServerEventName[], ServerEventMessage>(
  buildUrl(),
  SERVER_EVENT_NAMES,
  {
    withCredentials: true,
    autoReconnect: true,
    serializer: {
      read: raw => JSON.parse(raw ?? '') as ServerEventMessage,
    },
  },
))

export function useServerEvent<T extends ServerEventName>(
  eventName: T,
  handler: ServerEventHandler<T>,
) {
  if (import.meta.server) {
    return
  }

  const { data, event } = useSharedServerEvents()

  watch(data, (message) => {
    if (!message || event.value !== eventName || message.type !== eventName) {
      return
    }
    handler(message.payload as ServerEventPayloads[T])
  })
}
