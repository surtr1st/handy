import { computed } from 'vue';
import { defineStore } from 'pinia';
import { useLocalStorage } from '@vueuse/core';
import { SnakeBacklog, SnakeIteration } from '../types';

export const useIterationRoute = defineStore('iteration', () => {
  const id = useLocalStorage('iterationId', 0);
  const iteration = useLocalStorage('iteration', {});
  const iterationId = computed(() => id.value);
  const getIteration = computed(() => iteration.value as SnakeIteration);
  function setIterationId(value: number) {
    id.value = value;
  }
  function setIteration(_iteration: SnakeIteration) {
    iteration.value = _iteration;
  }
  return {
    iterationId,
    setIterationId,
    getIteration,
    setIteration,
  };
});

export const useBacklogRoute = defineStore('backlog', () => {
  const id = useLocalStorage('backlogId', 0);
  const backlogId = computed(() => id.value);
  function setBacklogId(value: number) {
    id.value = value;
  }
  return {
    backlogId,
    setBacklogId,
  };
});
