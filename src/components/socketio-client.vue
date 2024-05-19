<script setup lang="ts">
import {onMounted, ref} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import Textarea from 'primevue/textarea';

const logMsg = ref("");
const url = ref("http://192.168.186.1:9001");

const isRun = ref(false)

const startService = async () => {
    await invoke('start_server', { "host": "0.0.0.0", "port": 10010, "cburl": "", "wsurl": url.value });
    isRun.value = true;
}

const stopService = async () => {
    await invoke('stop_server');
    isRun.value = false;
}


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

async function confirmExit() {
    const shouldExit = await confirm("退出将无法使用服务，确定要退出吗？");
    if (shouldExit) {
        await invoke('confirm_exit');
    }
}

async function startExitEventListener() {
    await listen('request-exit', () => {
        confirmExit();
    });
}

onMounted(()=>{
  logMsg.value = "填写回调地址（不填写也可以，消息会显示在此处），然后点击【启动】\n";
  startSerialEventListener();
  startExitEventListener();
})

</script>

<template>
    <div class="flex flex-col">
        <div class="flex-none flex flex-row justify-evenly">
            <InputText type="text" class="w-80%" v-model="url" placeholder="输入回调地址" />
            <Button label="停止" v-show="isRun" @click="stopService" severity="danger" />
            <Button label="启动" v-show="!isRun" @click="startService" severity="success" />
        </div>
        <div class="flex-1 h-full mt-20px">
            <Textarea v-model="logMsg" rows="20" cols="80" readonly disabled />
        </div>
    </div>
    
</template>

<style scoped></style>