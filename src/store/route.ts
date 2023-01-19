import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

export const useIterationRoute = defineStore('iteration', () => {
  const id = ref<number>(0);
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
  const id = ref<number>(0);
  const backlogId = computed(() => id.value);
  function setBacklogId(value: number) {
    id.value = value;
  }
  return {
    backlogId,
    setBacklogId,
  };
});
