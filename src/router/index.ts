import { defineAsyncComponent } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';

const GetStarted = defineAsyncComponent(
	() => import('../views/GetStarted.vue'),
);
const Home = defineAsyncComponent(() => import('../views/Home.vue'));
const Iteration = defineAsyncComponent(() => import('../views/Iteration.vue'));

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
		component: Iteration,
	},
];

export const router = createRouter({
	history: createWebHistory(),
	routes,
});
