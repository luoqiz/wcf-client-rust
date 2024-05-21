<script setup>
import { useServiceStore } from "../stores/service";
const store = useServiceStore();

</script>

<template>
    <div class="flex flex-col p-32px gap-16px">
        <InputText type="text" class="w-80%" v-model="store.cburl" placeholder="输入http回调地址" />
        <InputText type="text" class="w-80%" v-model="store.wsurl" placeholder="输入ws回调地址" />
    </div>
</template>

<style scoped>
</style>