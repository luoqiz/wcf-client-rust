<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useServiceStore } from "../stores/service";
import { get_contacts } from "~/service/contact_service"
import { Contact } from "~/types/contact";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import SelectButton from 'primevue/selectbutton';

const store = useServiceStore();

const contacts = ref<Contact[]>([]);

onMounted(async () => {
  if (store.isRunning) {
    contacts.value = await get_contacts();
  }
})

const typeOptions = ref([
  { label: '好友', value: 1 },
  { label: '群组', value: 2 },
  { label: '公众号', value: 3 },
  { label: '全部', value: 0 }
]);

const selectedType = ref(1);

const showContacts = computed(() => {
  if (selectedType.value === 1) {
    return contacts.value.filter(item => { return item.wxid.startsWith('wxid_'); });
  }
  if (selectedType.value === 2) {
    return contacts.value.filter(item => { return item.wxid.endsWith('@chatroom'); });
  }
  if (selectedType.value === 3) {
    return contacts.value.filter(item => { return item.wxid.startsWith('gh_'); });
  }
  return contacts.value;
});

</script>

<template>
  <div v-if="store.isRunning" class="card flex justify-content-center">

    <div class="card">
      <div class="flex justify-center mb-6">
        <SelectButton v-model="selectedType" :options="typeOptions" optionLabel="label" dataKey="value"
          optionValue="value" />
      </div>
      <DataTable :value="showContacts" size="small" tableStyle="min-width: 50rem">
        <Column field="wxid" header="微信id" :maxConstraints=50></Column>
        <Column field="name" header="昵称" :maxConstraints=44></Column>
        <Column field="remark" header="备注"></Column>
      </DataTable>
    </div>
  </div>
  <div v-else>
    请先启动服务
  </div>
</template>

<style scoped></style>