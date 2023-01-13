import { createApp } from 'vue';
import 'vfonts/Inter.css';
import App from './App.vue';
import { router } from './router';

createApp(App).use(router).mount('#app');
