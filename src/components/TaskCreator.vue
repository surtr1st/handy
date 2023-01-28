<script setup lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { onMounted, onUnmounted, ref } from 'vue';
  import { useDebounceFn } from '@vueuse/core';
  import { Add28Filled } from '@vicons/fluent';
  import { useBacklogRoute, targetInvoked } from '../store';
  import { useMessages, participant } from '../constants';
  import {
    NSpace,
    NModal,
    NInput,
    NInputNumber,
    NGrid,
    NGi,
    NDatePicker,
    NButton,
    NIcon,
    NCard,
  } from 'naive-ui';

  const { backlogId } = useBacklogRoute();
  const { onSuccess, onError } = useMessages();
  const open = ref(false);
  const picAlias = ref('');
  const model = ref({
    name: '',
    startedDate: new Date().getTime(),
    hours: 0,
  });

  function addTask() {
    const fields = {
      name: model.value.name,
      created_date: new Date().getTime(),
      started_date: model.value.startedDate,
      hours: model.value.hours,
      worked_hours: 0,
      status: false,
      mode: false,
      participant_id: participant.id,
      backlog_id: backlogId,
    };
    invoke<string>('create_task', { fields })
      .then((message) => {
        onSuccess(message);
        open.value = false;
        targetInvoked.taskAction = !targetInvoked.taskAction;
        model.value = {
          name: '',
          startedDate: new Date().getTime(),
          hours: 0,
        };
      })
      .catch((message) => onError(message));
  }

  const debounceAddTask = useDebounceFn(addTask, 300);

  onMounted(() => {
    invoke<string>('find_participant_alias', { id: participant.id })
      .then((res) => (picAlias.value = res))
      .catch((e) => onError(e));
  });
  onUnmounted(() => {});
</script>

<template>
  <NSpace
    style="height: 85%"
    justify="center"
    align="center"
  >
    <NButton
      primary
      circle
      type="primary"
      size="large"
      @click="open = !open"
    >
      <NIcon>
        <Add28Filled />
      </NIcon>
    </NButton>
  </NSpace>
  <NModal
    :show="open"
    closable
  >
    <NCard style="width: 500px">
      <NSpace vertical>
        <NInput
          v-model:value="model.name"
          placeholder="Task Name"
        />
        <NGrid
          responsive="self"
          :cols="12"
          :x-gap="12"
        >
          <NGi :span="6">
            <NDatePicker
              v-model:value="model.startedDate"
              placeholder="Started Date"
            />
          </NGi>
          <NGi :span="6">
            <NInputNumber
              v-model:value="model.hours"
              placeholder="Hour"
            />
          </NGi>
        </NGrid>
        <NInput
          :value="picAlias"
          readonly
        />
        <NSpace justify="center">
          <NButton
            primary
            size="large"
            style="width: 150px"
            type="primary"
            @click="debounceAddTask"
          >
            Add
          </NButton>
          <NButton
            primary
            size="large"
            style="width: 150px"
            type="error"
            @click="open = false"
          >
            Cancel
          </NButton>
        </NSpace>
      </NSpace>
    </NCard>
  </NModal>
</template>
