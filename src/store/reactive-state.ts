import { reactive } from 'vue';
import { useSessionStorage } from '@vueuse/core';
import { SnakeBacklog } from '../types';

export const targetInvoked = reactive({
  backlogAction: false,
  taskAction: false,
  criteriaAcceptanceAction: false,
});

export const reviewStore = reactive({
  content: '',
});

export const retroStore = reactive({
  content: '',
});

export const backlogStore = reactive(
  useSessionStorage('backlog', {} as SnakeBacklog),
);
