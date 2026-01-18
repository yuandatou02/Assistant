import "./style.css";
import App from "./main.vue";
import router from "./router";
import {createApp} from "vue";

createApp(App).use(router).mount("#app");
