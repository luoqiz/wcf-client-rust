<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { nanoid } from 'nanoid'
import { useServiceStore } from "~/stores/service";
import { get_contacts } from "~/service/contact_service"
import { task_save } from "~/service/task_service"
import { Contact } from '~/types/contact';
import { ForwardTask } from '~/types/task';

const serviceStore = useServiceStore();

// 所有好友列表
const contacts = ref<Contact[]>([]);
// 所有参与的用户
const contact_select = ref<Contact[]>([]);
// 源用户
const contact_org = ref<Contact[]>([]);
// 目标用户
const contact_target = ref<Contact[]>([]);

watch(contact_select,()=>{
  contact_org.value = contact_select.value;
});

const current_task = ref<ForwardTask>({
  id: nanoid(),
  enabled: true,
  remark: '',
  from_wxid_list: [],
  to_wxid_list: []
})

const add_task = ()=>{
  current_task.value.from_wxid_list = contact_org.value.map(item => {return item.wxid});
  current_task.value.to_wxid_list = contact_target.value.map(item => {return item.wxid});
  task_save(serviceStore.userInfo!.wxid, current_task.value );
}

onMounted(async () => {
  if(serviceStore.isRunning){
    contacts.value = await get_contacts();
  }
});
 
</script>

<template>
  <div class="w-full flex flex-col gap-16px">
    <a-input :style="{width:'320px'}" placeholder="此任务名称" v-model="current_task.remark"/>
    <a-select class="w-full" placeholder="选择参与的用户" 
        multiple v-model="contact_select" :filter-option="true">
      <a-option v-for="item of contacts" :value="item">{{item.remark}}</a-option>
    </a-select>

    <div class="card">
        <a-transfer
          show-search
          :data="contact_org"
          v-model="contact_target"
          :source-input-search-props="{
            placeholder:'source item search'
          }"
          :target-input-search-props="{
            placeholder:'target item search'
          }"
        >
        </a-transfer>
    </div>
    <a-button type="primary" @click="add_task">保存</a-button>
  </div>
</template>
<style scoped></style>
