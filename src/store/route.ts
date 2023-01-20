import { computed } from 'vue';
import { defineStore } from 'pinia';
import { useLocalStorage } from '@vueuse/core';

export const useIterationRoute = defineStore('iteration', () => {
  const id = useLocalStorage('iterationId', 0);
  const iterationId = computed(() => id.value);
  function setIterationId(value: number) {
    id.value = value;
  }
  return {
    iterationId,
    setIterationId,
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
