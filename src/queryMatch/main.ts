import "./style.css";
import App from "./main.vue";
import {createApp} from "vue";
import {createPinia} from "pinia";

createApp(App).use(createPinia()).mount("#app");