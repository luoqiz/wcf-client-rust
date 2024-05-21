import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { listen } from "@tauri-apps/api/event";

export const useServiceStore = defineStore('service', () => {
    // 当前状态是否运行
    const isRunning = ref<boolean>(false);
    // http 回调地址
    const cburl = ref<string>('');
    // ws 服务器地址
    const wsurl = ref<string>('');
     
    // 启动服务
    const startService = async () => {
        await invoke('start_server', { "host": "0.0.0.0", "port": 10010, "cburl": cburl.value, "wsurl": wsurl.value });
        isRunning.value = true;
    }
    // 关闭服务
    const stopService = async () => {
        await invoke('stop_server');
        isRunning.value = false;
    }
    // 确认退出的弹窗
    async function confirmExit() {
        const shouldExit = await confirm("退出将无法使用服务，确定要退出吗？");
        if (shouldExit) {
            await invoke('confirm_exit');
        }
    }
    // 开启退出服务的监听
    async function startExitEventListener() {
        await listen('request-exit', () => {
            confirmExit();
        });
    }
  
    return { isRunning, cburl, wsurl, startService, stopService, startExitEventListener}
  })