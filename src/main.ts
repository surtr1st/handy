import '@vueup/vue-quill/dist/vue-quill.snow.css';
import '@vueup/vue-quill/dist/vue-quill.bubble.css';
import App from './App.vue';
import { createApp } from 'vue';
import { router } from './router';
import { pinia } from './store';

createApp(App).use(pinia).use(router).mount('#app');
