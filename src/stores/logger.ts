import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useLoggerStore = defineStore('logger', () => {
    const logMsg = ref("请在配置里填写回调地址（不填写也可以，消息会显示在此处），然后点击【启动】\n");

    const appendLogWithLimit = (message: any, maxLines = 1000) => {
        logMsg.value += message + "\n";
        let lines = logMsg.value.split("\n");

        if (lines.length > maxLines) {
            lines = lines.slice(lines.length - maxLines);
            logMsg.value = lines.join("\n");
        }
    }

    return { logMsg, appendLogWithLimit }
  })