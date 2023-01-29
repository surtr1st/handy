import { reactive } from 'vue';
import { useSessionStorage } from '@vueuse/core';
import { SnakeBacklog, SnakeTask } from '../types';

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

export const targetLogwork = reactive(
  useSessionStorage('targetLogwork', {
    taskId: 0,
    workedHours: 0,
  }),
);
