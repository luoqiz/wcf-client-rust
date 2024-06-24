import { createApp } from "vue";
import "./assets/main.css";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import "virtual:uno.css";
import { createRouter, createWebHistory } from 'vue-router/auto'

//in main.js
import "primevue/resources/themes/lara-light-green/theme.css";
// import 'primevue/resources/themes/saga-blue/theme.css';
import "primevue/resources/primevue.min.css";
import "primeicons/primeicons.css";

import ArcoVue from '@arco-design/web-vue';
import '@arco-design/web-vue/dist/arco.css';


import { createPinia } from 'pinia'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
})
const app = createApp(App);
app.use(PrimeVue, {
//   unstyled: true ,
  // ripple: true
});

const pinia = createPinia();
app.use(pinia);
app.use(ArcoVue);
app.use(router);
app.mount("#app");
