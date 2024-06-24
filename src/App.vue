<script setup lang="ts">
import { onMounted } from 'vue';
import Layout from './layout/layout.vue';
import { useConfigStore } from './stores/config';

let configStore = useConfigStore();

onMounted(async ()=>{
   await configStore.read();
})

</script>

<template>
   <Layout></Layout>
</template>

<style scoped></style>
