<script setup>
import { useConfigStore } from "../stores/config";
const configStore = useConfigStore();

const handleSubmit = (data) => {
    configStore.update();
};

</script>

<template>
    <div class="flex flex-col p-32px gap-16px justify-content-center">
        <a-form :model="configStore.kConfig" :style="{ width: '600px' }" @submit="handleSubmit">

            <a-card title="回调配置" :bordered="false" :style="{ width: '100%' }">
                <a-form-item field="cburl" label="http">
                <a-input
                    v-model="configStore.kConfig.cburl"
                    placeholder="请输入http回调地址"
                />
                </a-form-item>

                <a-form-item field="wsurl" label="socketIo">
                    <a-input v-model="configStore.kConfig.wsurl" placeholder="请输入ws回调地址" />
                </a-form-item>
            </a-card>

            <a-card title="文件配置" :bordered="false" :style="{ width: '100%' }">
                <a-form-item field="file_dir" label="保存地址">
                <a-input
                    v-model="configStore.kConfig.file_dir"
                    placeholder="请输入保存地址"
                />
                </a-form-item>

            </a-card>
          
            <a-form-item>
                <a-button html-type="submit">保存配置</a-button>
            </a-form-item>
        </a-form>
    </div>
</template>

<style scoped>
</style>