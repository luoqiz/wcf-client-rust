<script setup lang="ts">
import LeftMenu from "./left-menu.vue";
import Header from "./header.vue";
import { useRouter } from "vue-router";

const router = useRouter();

const handle_menu_click = (url: string) => {
  router.push(url);
}
</script>

<template>
  <div class="layout">
    <div class="header">
      <Header></Header>
    </div>
    <div class="body">
      <div class="side">
        <LeftMenu @handle_menu_click="handle_menu_click"></LeftMenu>
      </div>
      <div class="content">
        <router-view></router-view>
      </div>
    </div>
  </div>
</template>

<style scoped>
.layout {
  height: 100vh;
  display: flex;
  font-stretch: condensed;
  flex-direction: column;
}

.layout .header {
  height: 64px;
  justify-content: center;
  text-align: center;
  display: flex;
  flex-direction: column;
  font-size: 16px;
  font-stretch: condensed;
}

.layout .body {
  flex: 1;
  display: flex;
  flex-direction: row;

  .side {
    height: 100%;
  }

  .content {
    flex: 1;
    height: 100%;
    padding: 16px;
    overflow: hidden;
  }
}

.layout :deep(.arco-layout-content) {
  display: flex;
  flex-direction: column;
  font-size: 16px;
  background-color: blue;
  font-stretch: condensed;
  flex: auto;
  height: 100%;
}

/* .layout-right {
  overflow: hidden;
  background-color: yellow;
  flex: 1;
}

.layout-demo :deep(.arco-layout-sider-children),
.layout-demo :deep(.arco-layout-content) {
  display: flex;
  flex-direction: column;
  font-size: 16px;
  font-stretch: condensed;
} */
</style>
