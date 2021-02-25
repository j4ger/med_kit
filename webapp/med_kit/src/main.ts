import { createApp, h } from "vue";
import App from "./App.vue";
import WaveUI from "wave-ui";
import "wave-ui/dist/wave-ui.css";
import "@mdi/font/css/materialdesignicons.min.css";
import { createRouter, createWebHistory } from "vue-router";
import InitProduct from "./components/InitProduct.vue";
import FetchProfile from "./components/FetchProfile.vue";

const app = createApp({ render: () => h(App) });

const routes = [
  { name: "初始化产品", path: "/init", component: InitProduct },
  { name: "信息登记", path: "/fetch/:uuid", component: FetchProfile },
];
const router = createRouter({
  history: createWebHistory(),
  routes,
});
app.use(router);

new WaveUI(app);

app.mount("#app_renderer");
