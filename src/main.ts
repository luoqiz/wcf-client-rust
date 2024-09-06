import { createApp } from "vue";
import "./assets/main.css";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import "virtual:uno.css";
import "primeicons/primeicons.css";

import ArcoVue from "@arco-design/web-vue";
import "@arco-design/web-vue/dist/arco.css";
import ToastService from "primevue/toastservice";
import Aura from "@primevue/themes/aura";
import { createPinia } from "pinia";
import { createRouter, createWebHistory } from "vue-router";
import { routes } from "vue-router/auto-routes";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});
const app = createApp(App);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
  },
  //   unstyled: true ,
  // ripple: true
});
app.use(ToastService);
const pinia = createPinia();
app.use(pinia);
app.use(ArcoVue);
app.use(router);
app.mount("#app");
