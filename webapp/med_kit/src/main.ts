import { createApp, h } from "vue";
import App from "./App.vue";
import WaveUI from "wave-ui";
import "wave-ui/dist/wave-ui.css";
import "@mdi/font/css/materialdesignicons.min.css";
import VueRouter from "vue-router";

const app = createApp({ render: () => h(App) });

new WaveUI(app);
app.use(VueRouter);

app.mount("#app_renderer");
