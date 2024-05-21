<script setup lang="ts">
import Logger from '../components/logger.vue';
import Service from '../components/service.vue';
</script>

<template>
  <div class="h-full flex flex-col">
    <div class="flex-none">
      <Service />
    </div>
    <div class="flex-1 mt-32px">
      <Logger />
    </div>
  </div>
</template>

<style scoped></style>
