<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const logMsg = ref("");
const url = ref("");

const isRun = ref(false)

const startService = async () =>  {
    await invoke('start_server', { "host": "0.0.0.0", "port": 10010, "cburl": url.value });
    isRun.value = true;
}

const stopService = async () => {
    await invoke('stop_server');
    isRun.value = false;
}


function appendLogWithLimit(message:string, maxLines = 1000) {
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

window.addEventListener("DOMContentLoaded", () => {
    logMsg.value = "填写回调地址（不填写也可以，消息会显示在此处），然后点击【启动】\n";
    startSerialEventListener();
    startExitEventListener();
});

</script>

<template>
  <div class="row">
    <input class="greet-input" v-model="url" placeholder="输入回调地址" />
    <button type="submit" v-show="isRun" @click="stopService">停止</button>
    <button type="submit" v-show="!isRun" @click="startService">启动</button>
  </div>
  <div class="log-container">
    <textarea v-model="logMsg" rows="25" cols="110" readonly></textarea>
  </div>
</template>

<style scoped>
.log-container{
    margin-top: 36px;
}

.greet-input {
  margin-right: 5px;
  width: 70%;
}
</style>