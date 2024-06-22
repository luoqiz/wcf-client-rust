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

onMounted(()=>{
    startSerialEventListener();
})
</script>

<template>
    <div class="w-full h-full">
        <Textarea v-model="logStore.logMsg" rows="20" cols="72" readonly disabled />
    </div>
</template>

<style scoped></style>