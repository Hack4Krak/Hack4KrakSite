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
  <PanelCard body>
    <div v-if="variant === 'panel'" class="flex items-baseline justify-between gap-4 mb-5">
      <PanelSectionTitle>
        Drużyna
      </PanelSectionTitle>
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
        <PanelDataLabel>Nazwa drużyny</PanelDataLabel>
        <UInput
          v-model="teamNameDraft"
          :readonly="!editingName"
          variant="none"
          class="mt-1"
          :ui="{ base: `border-b-2 px-0 py-1 text-lg font-bold ${editingName ? 'border-primary' : 'border-transparent'}` }"
        />
      </label>
      <PanelActionButton
        tone="neutral"
        class="self-start sm:self-end"
        @click="editingName ? submitRename() : editingName = true"
      >
        {{ editingName ? 'Zapisz' : 'Zmień nazwę' }}
      </PanelActionButton>
    </form>

    <div v-else>
      <PanelDataLabel>
        Nazwa drużyny
      </PanelDataLabel>
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

        <div class="min-w-0">
          <div class="flex min-w-0 items-center gap-1.5">
            <p class="font-bold truncate">
              {{ member.name }}
            </p>
            <UTooltip v-if="member.is_leader" text="Lider drużyny">
              <UIcon
                name="i-pixel-crown"
                class="text-primary shrink-0 size-3.5"
              />
            </UTooltip>
          </div>
        </div>

        <UButton
          v-if="isLeader && !member.is_leader && !isCompact"
          color="error"
          variant="link"
          class="p-0 text-xs font-bold uppercase tracking-wider text-muted hover:text-error"
          aria-label="Wyrzuć z drużyny"
          @click="emit('kick', member.name)"
        >
          Usuń
        </UButton>
      </li>

      <li
        v-for="(invited, invitedIndex) in invitedUsersList"
        :key="`inv-${invited}`"
        class="grid grid-cols-[auto_1fr_auto] gap-4 items-center py-3"
      >
        <span class="text-xs text-muted/70 w-6 tabular-nums">
          {{ String(members.length + invitedIndex + 1).padStart(2, '0') }}
        </span>
        <div class="min-w-0">
          <div>
            <p class="font-bold truncate text-muted">
              {{ invited }}
            </p>
            <UBadge color="primary" variant="outline" size="sm">
              Zaproszenie wysłane
            </UBadge>
          </div>
        </div>
        <UButton
          v-if="isLeader && !isCompact"
          color="error"
          variant="link"
          class="p-0 text-xs font-bold uppercase tracking-wider text-muted hover:text-error"
          @click="emit('revokeInvite', invited)"
        >
          Cofnij
        </UButton>
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
      <UInput
        v-model="inviteInput"
        type="text"
        placeholder="np. lajkonik_z_kazimierza"
        class="flex-1"
      />
      <PanelActionButton
        type="submit"
        filled
      >
        Zaproś
      </PanelActionButton>
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
  </PanelCard>
</template>
