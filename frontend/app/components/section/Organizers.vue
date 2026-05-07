<script setup lang="ts">
import { ORGANIZERS } from '~~/content/organizers'
</script>

<template>
  <div class="flex flex-col gap-6">
    <section
      v-for="section in ORGANIZERS"
      :key="section.sectionName"
      class="flex flex-col w-full text-center items-center justify-center"
    >
      <h3 class="font-semibold">
        {{ section.sectionName.toUpperCase() }}
      </h3>
      <div class="w-full grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 my-6">
        <TeamMemberCard
          v-for="organizer in section.members"
          :key="organizer.email"
          :name="organizer.name"
          :email="organizer.email"
          :role="organizer.role"
          :github="organizer.github"
          :linkedin="organizer.linkedin"
          :highlight="section.sectionName === 'zarząd'"
          class="w-full"
        />
      </div>
    </section>
  </div>
</template>
