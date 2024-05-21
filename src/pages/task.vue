<script setup lang="ts">
import { ref } from 'vue';
import { nanoid } from 'nanoid'
import { invoke } from '@tauri-apps/api/core';

const task_list = ref<ForwardTask[]>([]);

const add_task = async () => {
  const forwardTask =  {
    id: nanoid(),
    from_wxid_list: [nanoid(), nanoid()],
    to_wxid_list: [nanoid(), nanoid()]
  }
  //new ForwardTask(nanoid(),[nanoid(), nanoid()],[nanoid(), nanoid()]);
  await invoke("write_wxid_task", {wxid: "wx_id1",task: forwardTask});
}

const read_wxid_task = async()=>{
  const result = await invoke("read_wxid_task", {wxid: "wx_id1"});
  task_list.value = result as ForwardTask[]
}

 
</script>

<template>
  <Button @click="add_task"> 保持测试 </Button>
  <Button @click="read_wxid_task"> 获取数据 </Button>
  <div v-for="item in task_list">
    {{ JSON.stringify(item) }}
  </div>
</template>

<style scoped></style>
