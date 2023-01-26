import '@vueup/vue-quill/dist/vue-quill.snow.css';
import '@vueup/vue-quill/dist/vue-quill.bubble.css';
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css';
import App from './App.vue';
import VueVirtualScroller from 'vue-virtual-scroller';
import { createApp } from 'vue';
import { router } from './router';
import { pinia } from './store';

createApp(App).use(pinia).use(router).use(VueVirtualScroller).mount('#app');
