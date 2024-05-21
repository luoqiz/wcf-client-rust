<script setup lang="ts">
import {onMounted, ref} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useServiceStore } from "../stores/service";

const store = useServiceStore();

const contacts = ref<Contact[]>([]);
const get_contacts = async () => {
  let result: BaseResponse<Contact[]> = await invoke('get_contacts');
  contacts.value = result.data;
}

onMounted(()=>{
    if(store.isRunning){
        get_contacts();
    }
})
</script>

<template>
    <div class="">
        <div v-for="contact in contacts">
            {{ JSON.stringify(contact) }}
        </div>
    </div>
</template>

<style scoped></style>