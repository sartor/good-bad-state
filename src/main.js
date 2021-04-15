import { createApp } from 'vue'
import App from './front/App.vue'
import router from './front/router'
import fetch from "@/front/extensions/fetch";

import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap";

const app = createApp(App);

app
    .use(router)
    .mixin(fetch)
    .mount('#app');
