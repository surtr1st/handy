import { reactive } from 'vue';
import { useLocalStorage } from '@vueuse/core';
import { Backlog } from '../types';

export const reviewStore = reactive({
  content: '',
});

export const retroStore = reactive({
  content: '',
});

export const backlogStore = reactive({
  id: useLocalStorage('backlogId', 0),
  title: useLocalStorage('backlogTitle', ''),
  description: useLocalStorage('backlogDesc', ''),
  goals: useLocalStorage('backlogGoals', ''),
  priority: useLocalStorage('backlogPriority', 0),
  hours: useLocalStorage('backlogHours', 0),
  points: useLocalStorage('backlogPoints', 0),
  createdDate: useLocalStorage('backlogDate', 0),
});
