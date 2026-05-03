<script setup lang="ts">
import { organizers } from '~~/content/organizers'
</script>

<template>
  <section v-for="sectionName in organizers.hierarchy" :key="sectionName" class="flex flex-col w-full text-center items-center justify-center mb-6">
    <h3 class="font-semibold font-neutral-50">
      {{ sectionName.toUpperCase() }}
    </h3>
    <div class="w-full grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 my-6">
      <TeamMemberCard v-for="organizer in organizers.team.filter(member => member.section === sectionName)" :key="organizer.name" :name="organizer.name" :email="organizer.email" :role="organizer.role" :github="organizer.github" :linkedin="organizer.linkedin" class="w-full" />
    </div>
  </section>
</template>

<style scoped>

</style>
