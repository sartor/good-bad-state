import { createApp } from 'vue'
import App from './front/App.vue'
import router from './front/router'
import fetch from "@/front/extensions/fetch";

const app = createApp(App);

app
    .use(router)
    .mixin(fetch)
    .mount('#app');
