import { defineAsyncComponent } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';

const GetStarted = defineAsyncComponent(
  () => import('../views/GetStarted.vue'),
);
const Home = defineAsyncComponent(() => import('../views/Home.vue'));
const Iterations = defineAsyncComponent(
  () => import('../views/Iterations.vue'),
);
const Iteration = defineAsyncComponent(() => import('../views/Iteration.vue'));
const Backlog = defineAsyncComponent(() => import('../views/Backlog.vue'));
const Tasks = defineAsyncComponent(() => import('../views/Tasks.vue'));

const routes = [
  {
    path: '/',
    component: GetStarted,
  },
  {
    path: '/home',
    component: Home,
  },
  {
    path: '/iterations',
    component: Iterations,
  },
  {
    path: '/iterations/:id',
    component: Iteration,
    children: [
      {
        path: 'backlogs/:bid',
        component: Backlog,
      },
      {
        path: 'backlogs/:bid/tasks',
        component: Tasks,
      },
    ],
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
