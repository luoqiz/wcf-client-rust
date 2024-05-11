import { createApp } from "vue";
import "./assets/main.css";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import "virtual:uno.css";

//in main.js
import "primevue/resources/themes/lara-light-green/theme.css";
// import 'primevue/resources/themes/saga-blue/theme.css';
import "primevue/resources/primevue.min.css";
import "primeicons/primeicons.css";

const app = createApp(App);
// app.use(PrimeVue, {
//   unstyled: true ,
//   ripple: true
// });
app.use(PrimeVue);

app.mount("#app");
