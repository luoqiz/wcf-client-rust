import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { KConfig } from '~/types/config';

export const useConfigStore = defineStore('config', () => {
    // 整个配置文件
    const kConfig = ref<KConfig>({
      cburl: '',
      wsurl: '',
      file_dir: '',
    });
     
    const update = async () => {
      console.log(kConfig.value);
      let res = await invoke("save_config", { config: kConfig.value });
      console.log(res)
    }

    const read = async ()=>{
      let res = await invoke("read_config");
      kConfig.value = res as KConfig;
    }

    return { kConfig, update, read}
  })