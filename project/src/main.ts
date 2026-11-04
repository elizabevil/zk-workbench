import { createPinia } from "pinia";
import { createApp } from "vue";

import "highlight.js/styles/github-dark.css";
import "./styles/global.css";

import { i18n } from "@/i18n";
import { router } from "@/router";
import App from "./App.vue";

createApp(App).use(createPinia()).use(i18n).use(router).mount("#app");
