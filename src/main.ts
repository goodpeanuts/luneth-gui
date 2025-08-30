import { createApp } from "vue";
import { attachConsole, info } from '@tauri-apps/plugin-log';
import App from "./App.vue";
import { clearExpiredImageCache } from './store/cache';

// 配置日志转发到Tauri后端
attachConsole().then(() => {
    info('frontend log init');
}).catch(err => {
    console.error('Failed to attach console:', err);
});

// 清理过期的图片缓存
clearExpiredImageCache();

// 设置定期清理过期缓存（每10分钟）
setInterval(clearExpiredImageCache, 10 * 60 * 1000);

createApp(App).mount("#app");
