<script setup lang="ts">
import type { ApiResponse } from '#open-fetch'

type Members = ApiResponse<'my_team'>['members']
type Member = Members[number]

interface Props {
  team: ApiResponse<'my_team'> | null | undefined
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
  <article class="panel-card">
    <header
      v-if="variant === 'panel'"
      class="panel-card-header border-b-2 border-surface-muted bg-surface-muted/20"
    >
      <h2 class="panel-card-title">
        Drużyna
      </h2>
      <span class="text-xs text-muted tabular-nums">
        {{ filledCount }}/{{ maxTeamSize }} osób
      </span>
    </header>

    <form
      v-if="isLeader"
      class="panel-card-body pb-0 flex flex-col sm:flex-row sm:items-end gap-3"
      @submit.prevent="submitRename"
    >
      <label class="flex-1">
        <span class="text-xs uppercase tracking-wider text-muted">Nazwa drużyny</span>
        <input
          v-model="teamNameDraft"
          :readonly="!editingName"
          class="block w-full mt-1 bg-transparent border-2 px-3 py-2 text-lg font-bold outline-none"
          :class="editingName ? 'border-primary' : 'border-transparent px-0'"
        >
      </label>
      <button
        type="button"
        class="px-4 py-2 border-2 border-surface-muted text-sm font-medium hover:border-primary transition-colors cursor-pointer"
        @click="editingName ? submitRename() : editingName = true"
      >
        {{ editingName ? 'Zapisz' : 'Zmień nazwę' }}
      </button>
    </form>

    <div v-else class="panel-card-body pb-0">
      <p class="text-xs uppercase tracking-wider text-muted">
        Nazwa drużyny
      </p>
      <p class="text-lg font-bold text-primary mt-1">
        {{ team?.team_name ?? '—' }}
      </p>
    </div>

    <ul class="panel-card-body space-y-3">
      <li
        v-for="(member, idx) in members"
        :key="member.name"
        class="panel-subcard grid grid-cols-[auto_1fr_auto] gap-4 items-center"
        :class="member.is_leader ? 'border-primary/50 bg-primary/5' : ''"
      >
        <span class="text-xs uppercase tracking-wider text-muted/80 w-6 tabular-nums">
          {{ String(idx + 1).padStart(2, '0') }}
        </span>

        <div class="flex items-center gap-3 min-w-0">
          <span
            class="size-9 border-2 flex items-center justify-center font-bold text-sm shrink-0"
            :class="member.is_leader
              ? 'border-primary text-primary bg-primary/10'
              : 'border-surface-muted text-default'"
          >
            {{ initials(member.name) }}
          </span>
          <div class="min-w-0">
            <p class="font-medium truncate">
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
          v-if="isLeader && !member.is_leader"
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
        class="panel-subcard grid grid-cols-[auto_1fr_auto] gap-4 items-center border-primary/40 border-dashed bg-primary/5"
      >
        <span class="text-xs uppercase tracking-wider text-muted/80 w-6 tabular-nums">
          {{ String(members.length + invitedIndex + 1).padStart(2, '0') }}
        </span>
        <div class="flex items-center gap-3 min-w-0">
          <span class="size-9 border-2 border-dashed border-primary/40 text-primary/60 flex items-center justify-center font-bold text-sm shrink-0">
            {{ initials(invited) }}
          </span>
          <div class="min-w-0">
            <p class="font-medium truncate text-muted">
              @{{ invited }}
            </p>
            <p class="text-xs mt-0.5 uppercase tracking-wider text-primary">
              Zaproszenie wysłane
            </p>
          </div>
        </div>
        <button
          v-if="isLeader"
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
        class="panel-subcard grid grid-cols-[auto_1fr_auto] gap-4 items-center opacity-50 border-dashed"
      >
        <span class="text-xs uppercase tracking-wider text-muted/60 w-6 tabular-nums">
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
      v-if="isLeader && freeSlots > 0"
      class="px-5 pb-3 flex items-stretch gap-2"
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
      v-if="inviteError"
      class="px-5 pb-3 text-xs text-error"
    >
      {{ inviteError }}
    </p>
    <p
      v-else-if="isLeader && freeSlots > 0"
      class="px-5 pb-3 text-xs text-muted"
    >
      Uczestnik musi mieć konto na hack4krak.pl
    </p>
  </article>
</template>
