<script setup lang="ts">
import LeftMenu from "./left-menu.vue";
import { useRouter } from "vue-router";

const router = useRouter();

const handle_menu_click = (url:string)=>{
    router.push(url);
}

</script>

<template>
  <div class="h-screen flex flex-row">
    <div class="flex-none w-242px h-full border-l-0 border-t-0 border-b-0 border-r-4 border-zinc-200 border-solid">
      <LeftMenu @handle_menu_click="handle_menu_click"></LeftMenu> 
    </div>
    <div class="flex-auto p-16px h-full w-full">
      <ScrollPanel class="h-full w-full">
        <router-view></router-view>
      </ScrollPanel>
    </div>
    
  </div>
</template>

<style scoped></style>
