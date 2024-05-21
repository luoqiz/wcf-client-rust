<script setup lang="ts">
import {onMounted, ref} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useServiceStore } from "../stores/service";

const store = useServiceStore();

const contacts = ref<Contact[]>([]);
const get_contacts = async () => {
  let result: BaseResponse<{ [key: string]: any }> = await invoke('get_contacts');
  let contacts_list  = result.data["contacts"] as Contact[];
  contacts.value = contacts_list;
}

const selectedCountry = ref();

onMounted(()=>{
    if(store.isRunning){
        get_contacts();
    }
})
</script>

<template>
    <div v-if="store.isRunning" class="card flex justify-content-center">
        <Listbox v-model="selectedCountry" :options="contacts" optionLabel="name" optionValue="wxid">
            <template #option="slotProps">
                <Card>
                    <template #subtitle>
                        <div v-if="slotProps.option.remark">
                            {{ slotProps.option.remark }}
                        </div>
                        <div v-else>
                            {{ slotProps.option.name }}
                        </div>
                    </template>
                    <template #content>
                        <p class="m-0">
                            <div class="flex flex-col">
                                <div>昵称:  {{ slotProps.option.name }}</div>
                                <div>wxid:  {{ slotProps.option.wxid }}</div>
                            </div>
                        </p>
                    </template>
                </Card>
            </template>    
        </Listbox>
    </div>
    <div v-else>
        请先启动服务
    </div>
</template>

<style scoped></style>