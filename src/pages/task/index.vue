<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from "vue-router";
import { ForwardTask } from '~/types/task';
import { useServiceStore } from "~/stores/service";

const router = useRouter();
const serviceStore = useServiceStore();

const task_list = ref<ForwardTask[]>([]);

const add_task = async () => {
  router.push("/task/edit")
}

const read_wxid_task = async()=>{
  if(serviceStore.isRunning){
    const result = await invoke("read_wxid_task", {wxid: serviceStore.userInfo?.wxid});
    task_list.value = result as ForwardTask[]
  }
}

onMounted(async () => {
  await read_wxid_task();
});
 
</script>

<template>
  <div v-if="serviceStore.isRunning" class="card">
     
      <DataTable :value="task_list" tableStyle="width: 100%">
        <template #header>
            <div class="flex flex-wrap align-items-center justify-content-between gap-2">
                <span class="text-xl text-900 font-bold">任务列表</span>
                <Button icon="pi pi-refresh" rounded raised @click="read_wxid_task" />
                <Button icon="pi pi-plus" rounded raised @click="add_task" />
            </div>
        </template>
        <Column field="id" header="标识"></Column>
        <Column field="enabled" header="是否启用"></Column>
        <Column field="remark" header="备注"></Column>
      </DataTable>
  </div>
  <div v-else>
    请先启动服务
  </div>
</template>

<style scoped></style>
