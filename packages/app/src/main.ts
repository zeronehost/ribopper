import "sober";
import "./assets/scss/index.scss";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./routes";
import { store, useSubscribe } from "./stores";

createApp(App).use(store).use(useSubscribe).use(router).mount("#app");
