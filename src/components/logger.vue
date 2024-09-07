<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
// import Textarea from 'primevue/textarea';
import { useLoggerStore } from "../stores/logger";

import { VAceEditor } from 'vue3-ace-editor';
import 'ace-builds/src-noconflict/mode-json'; // Load the language definition file used below
import 'ace-builds/src-noconflict/theme-chrome'; // Load the theme definition file used below // Load the theme definition file used below

const logStore = useLoggerStore();

async function startSerialEventListener() {
  await listen('log-message', (logMessage) => {
    logStore.appendLogWithLimit(logMessage.payload);
  });
}
const options: any = ref({
  useWorker: true, // 启用语法检查,必须为true
  enableBasicAutocompletion: false, // 自动补全
  enableLiveAutocompletion: false, // 智能补全
  enableSnippets: false, // 启用代码段
  showPrintMargin: false, // 去掉灰色的线，printMarginColumn
  highlightActiveLine: true, // 高亮行
  highlightSelectedWord: true, // 高亮选中的字符
  tabSize: 4, // tab锁进字符
  fontSize: 14, // 设置字号
  wrap: false, // 是否换行
  readonly: true, // 是否可编辑
});

onMounted(() => {
  startSerialEventListener();
})
</script>

<template>
  <v-ace-editor ref="aceRef" v-model:value="logStore.logMsg" lang="text" :options="options" theme="chrome"
    class="w-full h-full" />
  <!-- <Textarea v-model="logStore.logMsg" rows="20" fluid="true" readonly disabled class="w-full h-full" /> -->
</template>

<style scoped></style>