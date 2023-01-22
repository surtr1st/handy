import { createRouter, createWebHistory } from 'vue-router';

const Login = () => import('../views/Login.vue');
const Register = () => import('../views/Register.vue');
const GetStarted = () => import('../views/GetStarted.vue');
const Home = () => import('../views/Home.vue');
const CreateIteration = () => import('../views/CreateIteration.vue');
const Iterations = () => import('../views/Iterations.vue');
const ReviewNRetro = () => import('../views/ReviewNRetro.vue');
const Me = () => import('../views/Me.vue');
const Success = () => import('../views/Success.vue');
const Iteration = () => import('../views/Iteration.vue');
const Backlog = () => import('../views/Backlog.vue');
const Tasks = () => import('../views/Tasks.vue');

const routes = [
  {
    path: '/',
    component: Login,
  },
  {
    path: '/signin',
    component: Login,
  },
  {
    path: '/signup',
    component: Register,
  },
  {
    path: '/mainpage',
    component: Home,
    children: [
      {
        path: 'getting-started',
        component: GetStarted,
      },
      {
        path: 'create-iteration',
        component: CreateIteration,
      },
      {
        path: 'iterations',
        component: Iterations,
      },
      {
        path: 'iterations/:iid',
        component: Iteration,
        props: true,
        children: [
          {
            path: 'backlogs/:bid',
            component: Backlog,
            props: true,
          },
          {
            path: 'backlogs/:bid/tasks',
            component: Tasks,
            props: true,
          },
        ],
      },
      {
        path: 'end-of-iteration',
        component: ReviewNRetro,
      },
      {
        path: 'me',
        component: Me,
      },
      {
        path: 'review/retro/completed/200',
        component: Success,
      },
    ],
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
