<script setup lang="ts">
import { ref } from "vue";
import { useServiceStore } from "~/stores/service"

const serviceStore = useServiceStore();
const items = ref([
  {
    separator: true
  },
  // {
  //   label: '查看',
  //   items: [

  //   ]
  // },
  {
    //label: '功能',
    items: [
      {
        label: '日志',
        icon: 'pi pi-home',
        command: () => { handle_menu_click("/") },
      },
      {
        label: '好友',
        icon: 'pi pi-users',
        command: () => { handle_menu_click("/contact") },
      },
      {
        label: '配置',
        icon: 'pi pi-cog',
        command: () => { handle_menu_click("/config") }
      },
      // {
      //   label: '任务',
      //   icon: 'pi pi-receipt',
      //   url: "/task",
      //   command: () => { handle_menu_click("/task") }
      // },
      {
        label: '消息',
        icon: 'pi pi-inbox',
        badge: 0,
        command: () => { handle_menu_click("/msg") }
      },

      // {
      //     label: 'Logout',
      //     icon: 'pi pi-sign-out',
      //     shortcut: '⌘+Q',
      //     url: "/task",
      //     action: (item: any)=>{
      //         handle_menu_click(item.url);
      //     }
      // }
    ]
  },
  {
    separator: true
  }
]);

const emit = defineEmits(['handle_menu_click'])
const handle_menu_click = (url: string) => {

  emit('handle_menu_click', url);
}
</script>
<template>
  <div class="h-full flex flex-col justify-content-center bg-light-50">
    <div class="flex-1 w-200px md:w-15rem">
      <Menu :model="items" class="flex-1 w-full md:w-15rem">
        <template #submenuheader="{ item }">
          <span class="text-primary font-bold">{{ item.label }}</span>
        </template>
        <template #item="{ item, props }">
          <div class="flex-auto">
            <a class="flex align-items-center" v-bind="props.action">
              <span :class="item.icon" />
              <span class="ml-2">{{ item.label }}</span>
              <Badge v-if="item.badge" class="ml-auto" :value="item.badge" />
              <span v-if="item.shortcut" class="ml-auto border-1 surface-border border-round surface-100 text-xs p-1">
                {{ item.shortcut }}
              </span>
            </a>
          </div>
        </template>
      </Menu>
    </div>

    <div class="flex-none">
      <div v-if="serviceStore.userInfo != null"
        class="relative overflow-hidden w-full p-link flex align-items-center p-2 pl-3 text-color hover:surface-200 border-noround">
        <Avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" class="mr-2" shape="circle" />
        <span class="flex flex-col">
          <span class="font-bold">{{ serviceStore.userInfo.name }}</span>
          <span class="text-sm">{{ serviceStore.userInfo.wxid }}</span>
        </span>
      </div>
      <div v-else class="flex-none p-2 text-sm text-color">
        尚未登录
      </div>
    </div>
  </div>
</template>
