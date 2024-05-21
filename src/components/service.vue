<script setup lang="ts">
import {onMounted, ref} from "vue";
import { listen } from "@tauri-apps/api/event";

import Button from 'primevue/button';
import { useServiceStore } from "../stores/service";

const store = useServiceStore();

const logMsg = ref("");

function appendLogWithLimit(message: any, maxLines = 1000) {
    logMsg.value += message + "\n";
    let lines = logMsg.value.split("\n");
    if (lines.length > maxLines) {
        lines = lines.slice(lines.length - maxLines);
        logMsg.value = lines.join("\n");
    }
}

async function startSerialEventListener() {
    await listen('log-message', (logMessage) => {
        appendLogWithLimit(logMessage.payload);
    });
}

onMounted(()=>{
  logMsg.value = "填写回调地址（不填写也可以，消息会显示在此处），然后点击【启动】\n";
  startSerialEventListener();
})
</script>

<template>
    <div class="flex flex-row">
        <Button label="停止" v-show="store.isRunning" @click="store.stopService" severity="success" icon="pi pi-pause" size="small"/>
        <Button label="启动" v-show="!store.isRunning" @click="store.startService" icon="pi pi-play" severity="warning" size="small"/>
    </div>
</template>

<style scoped></style>