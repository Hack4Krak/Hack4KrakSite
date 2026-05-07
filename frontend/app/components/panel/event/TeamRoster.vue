<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

type MyTeam = NonNullable<ApiResponse<'my_team'>>
type Members = MyTeam['members']
type Member = Members[number]

interface Props {
  team: MyTeam | null | undefined
  invitedUsers?: string[] | null
  isLeader?: boolean
  variant?: 'panel' | 'compact'
  maxTeamSize?: number
}

const props = withDefaults(defineProps<Props>(), {
  invitedUsers: () => [],
  isLeader: false,
  variant: 'panel',
  maxTeamSize: 5,
})

const emit = defineEmits<{
  invite: [username: string]
  kick: [username: string]
  revokeInvite: [username: string]
  rename: [name: string]
}>()

const NAME_SPLIT = /[\s_-]+/
const LEADING_AT = /^@/

const inviteInput = ref('')
const inviteError = ref('')
const editingName = ref(false)
const teamNameDraft = ref('')

const members = computed<Member[]>(() => (props.team?.members ?? []) as Member[])
const invitedUsersList = computed(() => Array.isArray(props.invitedUsers) ? props.invitedUsers : [])
const filledCount = computed(() => members.value.length + invitedUsersList.value.length)
const freeSlots = computed(() => Math.max(0, props.maxTeamSize - filledCount.value))
const isCompact = computed(() => props.variant === 'compact')

watch(() => props.team?.team_name, (name) => {
  teamNameDraft.value = name ?? ''
}, { immediate: true })

function initials(name: string) {
  return name
    .split(NAME_SPLIT)
    .filter(Boolean)
    .slice(0, 2)
    .map(s => s[0]!.toUpperCase())
    .join('') || name.slice(0, 2).toUpperCase()
}

function submitInvite() {
  inviteError.value = ''
  const username = inviteInput.value.trim().replace(LEADING_AT, '')
  if (!username) {
    inviteError.value = 'Podaj nick uczestnika'
    return
  }
  if (filledCount.value >= props.maxTeamSize) {
    inviteError.value = 'Drużyna pełna'
    return
  }
  if (members.value.some(m => m.name === username) || invitedUsersList.value.includes(username)) {
    inviteError.value = 'Ten uczestnik jest już w drużynie'
    return
  }
  emit('invite', username)
  inviteInput.value = ''
}

function submitRename() {
  const name = teamNameDraft.value.trim()
  if (!name || name === props.team?.team_name) {
    editingName.value = false
    return
  }
  emit('rename', name)
  editingName.value = false
}
</script>

<template>
  <article class="panel-card panel-card-body">
    <div v-if="variant === 'panel'" class="flex items-baseline justify-between gap-4 mb-5">
      <p class="panel-section-title">
        Drużyna
      </p>
      <span class="text-xs text-muted tabular-nums">
        {{ filledCount }}/{{ maxTeamSize }} osób
      </span>
    </div>

    <form
      v-if="isLeader && !isCompact"
      class="flex flex-col sm:flex-row sm:items-end gap-3"
      @submit.prevent="submitRename"
    >
      <label class="flex-1">
        <span class="panel-data-label">Nazwa drużyny</span>
        <input
          v-model="teamNameDraft"
          :readonly="!editingName"
          class="block w-full mt-1 bg-transparent border-b-2 px-0 py-1 text-lg font-bold outline-none transition-colors"
          :class="editingName ? 'border-primary' : 'border-transparent'"
        >
      </label>
      <button
        type="button"
        class="px-4 py-2 border-2 border-surface-muted text-sm font-medium hover:border-primary transition-colors cursor-pointer self-start sm:self-end"
        @click="editingName ? submitRename() : editingName = true"
      >
        {{ editingName ? 'Zapisz' : 'Zmień nazwę' }}
      </button>
    </form>

    <div v-else>
      <p class="panel-data-label">
        Nazwa drużyny
      </p>
      <p class="text-lg font-bold text-primary mt-1">
        {{ team?.team_name ?? '—' }}
      </p>
    </div>

    <ul class="mt-5 divide-y divide-surface-muted/60 border-t border-surface-muted/60">
      <li
        v-for="(member, idx) in members"
        :key="member.name"
        class="grid grid-cols-[auto_1fr_auto] gap-4 items-center py-3"
      >
        <span class="text-xs text-muted/70 w-6 tabular-nums">
          {{ String(idx + 1).padStart(2, '0') }}
        </span>

        <div class="flex items-center gap-3 min-w-0">
          <span
            class="size-9 border-2 flex items-center justify-center font-pixelify text-base leading-none shrink-0"
            :class="member.is_leader
              ? 'border-primary text-primary bg-primary/10'
              : 'border-surface-muted text-default'"
          >
            {{ initials(member.name) }}
          </span>
          <div class="min-w-0">
            <p class="font-bold truncate">
              {{ member.name }}
            </p>
            <p
              class="text-xs mt-0.5 uppercase tracking-wider"
              :class="member.is_leader ? 'text-primary' : 'text-muted'"
            >
              <slot name="member-meta" :member="member">
                {{ member.is_leader ? 'Lider' : 'Członek' }}
              </slot>
            </p>
          </div>
        </div>

        <button
          v-if="isLeader && !member.is_leader && !isCompact"
          type="button"
          class="text-xs font-bold uppercase tracking-wider text-muted hover:text-error transition-colors cursor-pointer"
          aria-label="Wyrzuć z drużyny"
          @click="emit('kick', member.name)"
        >
          Usuń
        </button>
      </li>

      <li
        v-for="(invited, invitedIndex) in invitedUsersList"
        :key="`inv-${invited}`"
        class="grid grid-cols-[auto_1fr_auto] gap-4 items-center py-3"
      >
        <span class="text-xs text-muted/70 w-6 tabular-nums">
          {{ String(members.length + invitedIndex + 1).padStart(2, '0') }}
        </span>
        <div class="flex items-center gap-3 min-w-0">
          <span class="size-9 border-2 border-dashed border-primary/40 text-primary/60 flex items-center justify-center font-pixelify text-base leading-none shrink-0">
            {{ initials(invited) }}
          </span>
          <div class="min-w-0">
            <p class="font-bold truncate text-muted">
              {{ invited }}
            </p>
            <p class="text-xs mt-0.5 uppercase tracking-wider text-primary">
              Zaproszenie wysłane
            </p>
          </div>
        </div>
        <button
          v-if="isLeader && !isCompact"
          type="button"
          class="text-xs font-bold uppercase tracking-wider text-muted hover:text-error transition-colors cursor-pointer"
          @click="emit('revokeInvite', invited)"
        >
          Cofnij
        </button>
      </li>

      <li
        v-for="slot in freeSlots"
        :key="`empty-${slot}`"
        class="grid grid-cols-[auto_1fr_auto] gap-4 items-center py-3 opacity-60"
      >
        <span class="text-xs text-muted/50 w-6 tabular-nums">
          {{ String(filledCount + slot).padStart(2, '0') }}
        </span>
        <span class="text-xs uppercase tracking-wider text-muted/60">
          Wolne miejsce
        </span>
        <span class="text-xs text-muted/40">
          —
        </span>
      </li>
    </ul>

    <form
      v-if="isLeader && freeSlots > 0 && !isCompact"
      class="pt-4 mt-2 flex items-stretch gap-2 border-t border-surface-muted/60"
      @submit.prevent="submitInvite"
    >
      <input
        v-model="inviteInput"
        type="text"
        placeholder="nick uczestnika"
        class="flex-1 bg-transparent border-2 border-surface-muted px-3 py-2 text-sm outline-none focus:border-primary placeholder:text-muted/60"
      >
      <button
        type="submit"
        class="px-4 py-2 bg-primary hover:bg-secondary text-default text-sm font-bold uppercase tracking-wider cursor-pointer transition-colors"
      >
        Zaproś
      </button>
    </form>
    <p
      v-if="inviteError && !isCompact"
      class="mt-2 text-xs text-error"
    >
      {{ inviteError }}
    </p>
    <p
      v-else-if="isLeader && freeSlots > 0 && !isCompact"
      class="mt-2 text-xs text-muted"
    >
      Uczestnik musi mieć konto na hack4krak.pl
    </p>
    <div v-if="$slots.footer" class="pt-4 mt-2 border-t border-surface-muted/60">
      <slot name="footer" />
    </div>
  </article>
</template>
