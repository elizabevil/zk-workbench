import { createPinia } from "pinia";
import { createApp } from "vue";

import "./styles/global.css";

import { i18n } from "@/i18n";
import { router } from "@/router";
import App from "./App.vue";

createApp(App).use(createPinia()).use(i18n).use(router).mount("#app");
