<script setup lang="ts">
import { onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import Textarea from 'primevue/textarea';
import { useLoggerStore } from "../stores/logger";

const logStore = useLoggerStore();

async function startSerialEventListener() {
    await listen('log-message', (logMessage) => {
        logStore.appendLogWithLimit(logMessage.payload);
    });
}

onMounted(() => {
    startSerialEventListener();
})
</script>

<template>
    <Textarea v-model="logStore.logMsg" rows="20" cols="72" readonly disabled class="w-full h-full" />
</template>

<style scoped></style>