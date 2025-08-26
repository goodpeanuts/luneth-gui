import { createApp } from "vue";
import { attachConsole, info } from '@tauri-apps/plugin-log';
import App from "./App.vue";

// 配置日志转发到Tauri后端
attachConsole().then(() => {
    info('frontend log init');
}).catch(err => {
    console.error('Failed to attach console:', err);
});

createApp(App).mount("#app");
