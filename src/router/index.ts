import { defineAsyncComponent } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';

const GetStarted = defineAsyncComponent(
  () => import('../views/GetStarted.vue'),
);
const Home = defineAsyncComponent(() => import('../views/Home.vue'));
const Iterations = defineAsyncComponent(
  () => import('../views/Iterations.vue'),
);
const Iteration = defineAsyncComponent(
  () => import('../components/Iteration.vue'),
);

const routes = [
  {
    path: '/',
    name: 'getting-started',
    component: GetStarted,
  },
  {
    path: '/home',
    name: 'home',
    component: Home,
  },
  {
    path: '/iterations',
    component: Iterations,
  },
  {
    path: '/iterations/:id',
    component: Iteration,
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
