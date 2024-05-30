<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { nanoid } from 'nanoid'
import { useServiceStore } from "~/stores/service";
import { get_contacts } from "~/service/contact_service"
import { task_save } from "~/service/task_service"
import { Contact } from '~/types/contact';
import { ForwardTask } from '~/types/task';

const serviceStore = useServiceStore();

// type ContactSelect = Contact & {
//     select: boolean = false;
// }

// 所有好友列表
const contacts = ref<Contact[]>([]);
// 所有参与的用户
const contact_select = ref<Contact[]>([]);
 
// 绑定
const contacts_option = ref<Array<Contact[]>>([]);
contacts_option.value = [[],[]];

watch(contact_select,()=>{
  contacts_option.value[0] = contact_select.value;
});

const current_task = ref<ForwardTask>({
  id: nanoid(),
  enabled: true,
  remark: '',
  from_wxid_list: [],
  to_wxid_list: []
})

const add_task = ()=>{
  current_task.value.from_wxid_list = contacts_option.value[0].map(item => {return item.wxid});
  current_task.value.to_wxid_list = contacts_option.value[1].map(item => {return item.wxid});
  task_save(serviceStore.userInfo!.wxid, current_task.value );
}

onMounted(async () => {
  if(serviceStore.isRunning){
    contacts.value = await get_contacts();
  }
});
 
</script>

<template>
  <div class="w-full">
    <Button @click="add_task"> 保存 </Button>
    <InputText type="text" v-model="current_task.remark" />
    <MultiSelect v-model="contact_select" 
        :options="contacts" filter optionLabel="remark" placeholder="选择参与的用户"
        :maxSelectedLabels="3" class="w-full md:w-20rem" />
    <div class="card">
        <PickList v-model="contacts_option" listStyle="height:342px" dataKey="id" breakpoint="1400px">
            <template #sourceheader> 源信息用户 </template>
            <template #targetheader> 目标用户 </template>
            <template #item="slotProps">
              <div class="flex flex-column gap-2">
                  <span class="font-bold">{{ slotProps.item.name }}</span>
                  <div class="flex align-items-center gap-2">
                      <!-- <i class="pi pi-tag text-sm"></i> -->
                      --- <span>{{ slotProps.item.wxid }}</span>
                  </div>
              </div>
            </template>
        </PickList>
    </div>
  </div>
</template>
<style scoped></style>
