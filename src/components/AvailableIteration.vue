<script setup lang="ts">
  import { defineAsyncComponent, h, onMounted, onUnmounted, ref } from 'vue';
  import { invoke } from '@tauri-apps/api';
  import { SnakeIteration } from '../types';
  import { useFormattedDate } from '../constants';
  import { useDebounceFn } from '@vueuse/core';
  import { useMessages } from '../constants';
  import {
    NCard,
    NThing,
    NSpace,
    NStatistic,
    NButton,
    NGrid,
    NGi,
    NScrollbar,
  } from 'naive-ui';

  const Empty = defineAsyncComponent(() => import('../components/Empty.vue'));
  const iterations = ref<Array<SnakeIteration>>([]);
  const { onSuccess, onError } = useMessages();

  function joinIteration(id: number) {
    invoke<string>('join_iteration', {
      iterationId: id,
      participantId: parseInt(localStorage.getItem('PARTICIPANT_ID') as string),
    })
      .then((message) => onSuccess(message))
      .catch((message) => onError(message));
  }

  const debounceJoin = useDebounceFn(joinIteration);

  onMounted(() => {
    invoke<Array<SnakeIteration>>('get_iterations')
      .then((res) => (iterations.value = res))
      .catch((e) => console.log(e));
  });
  onUnmounted(() => (iterations.value = []));
</script>

<template>
  <Empty v-if="iterations.length < 1" />
  <NGrid
    v-else
    cols="3"
  >
    <NGi
      span="1"
      v-for="iteration in iterations"
    >
      <NCard
        :title="iteration.title"
        hoverable
        style="width: 430px"
      >
        <NThing content-style="font-size: 18px">
          <NGrid :cols="4">
            <NGi :span="2">
              <NSpace vertical>
                <NStatistic
                  label="Id"
                  :value="`#${iteration.id}`"
                />

                <NStatistic
                  label="Created By"
                  :value="iteration.created_by"
                />
                <NStatistic
                  label="Start Date"
                  :value="useFormattedDate(iteration?.start_date)"
                />
                <NStatistic
                  label="End Date"
                  :value="useFormattedDate(iteration?.end_date)"
                />
                <NButton
                  primary
                  type="primary"
                  style="width: 150px"
                  @click="debounceJoin(iteration.id)"
                >
                  Join
                </NButton>
              </NSpace>
            </NGi>
            <NGi :span="2">
              <NStatistic label="Description">
                <NScrollbar style="max-height: 300px">
                  <NCard>{{ iteration?.goals }} </NCard>
                </NScrollbar>
              </NStatistic>
            </NGi>
          </NGrid>
        </NThing>
      </NCard>
    </NGi>
  </NGrid>
</template>
