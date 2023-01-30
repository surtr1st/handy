import { computed } from 'vue';
import { defineStore } from 'pinia';
import { useSessionStorage } from '@vueuse/core';
import { SnakeIteration } from '../types';

export const useIterationRoute = defineStore('iteration', () => {
  const id = useSessionStorage('iterationId', 0);
  const iteration = useSessionStorage('iteration', {});
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
  const id = useSessionStorage('backlogId', 0);
  const backlogId = computed(() => id.value);
  function setBacklogId(value: number) {
    id.value = value;
  }
  return {
    backlogId,
    setBacklogId,
  };
});
