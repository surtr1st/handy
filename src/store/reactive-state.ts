import { reactive } from 'vue';
import { useLocalStorage } from '@vueuse/core';
import { SnakeBacklog } from '../types';

export const reviewStore = reactive({
  content: '',
});

export const retroStore = reactive({
  content: '',
});

export const backlogStore = reactive(
  useLocalStorage('backlog', {} as SnakeBacklog),
);
