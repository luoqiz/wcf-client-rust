<script setup lang="ts">
import HttpServer from "./components/http-server.vue";
import SocketIoClient from "./components/socketio-client.vue";

import TabView from 'primevue/tabview';
import TabPanel from 'primevue/tabpanel';

</script>

<template>
  <div class="container">
    <TabView>
      <TabPanel header="回调配置">
        <HttpServer />
      </TabPanel>
      <TabPanel header="socketio配置">
        <SocketIoClient />
      </TabPanel>
    </TabView>
  </div>

</template>

<style scoped></style>
