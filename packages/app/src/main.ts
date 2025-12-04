import "sober";
import "./assets/scss/index.scss";
import { createApp } from "vue";
import { store } from "./stores";
import App from "./App.vue";
import router from "./routes";

createApp(App)
.use(store)
.use(router)
.mount("#app");
