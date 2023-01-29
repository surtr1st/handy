<script setup lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { useDebounceFn } from '@vueuse/core';
  import { onMounted, ref } from 'vue';
  import { DEBOUNCE_TIME, useMessages, useNotifications } from '../helpers';
  import { targetInvoked, useIterationRoute } from '../store';
  import { BacklogType, SnakeBacklog } from '../types';
  import {
    NSelect,
    NCard,
    NModal,
    NGrid,
    NGi,
    NForm,
    NFormItemGi,
    NInput,
    NInputNumber,
    NSpace,
    NButton,
  } from 'naive-ui';

  const { iterationId } = useIterationRoute();
  const { onSuccess, onError } = useMessages();
  const { notifyError } = useNotifications();

  const open = ref(false);
  const form = ref();
  const backlogTypes = ref<Array<BacklogType>>([]);
  const model = ref({
    title: '',
    priority: 0,
    goals: '',
    description: '',
    hours: 0,
    points: 0,
    type: 1,
  });

  function createBacklog() {
    const fields = {
      title: model.value.title,
      priority: model.value.priority,
      goals: model.value.goals,
      description: model.value.description,
      hours: model.value.hours,
      points: model.value.points,
      created_date: new Date().getTime(),
      iteration_id: iterationId,
      progress_id: 1, // default: Undone
      type_id: model.value.type,
    };
    invoke<string>('create_backlog', { fields })
      .then((message) => {
        onSuccess(message);
        open.value = false;
        targetInvoked.backlogAction = !targetInvoked.backlogAction;
      })
      .catch((message) => onError(message));
  }
  const debounceCreateBacklog = useDebounceFn(createBacklog, DEBOUNCE_TIME);

  onMounted(() => {
    invoke<Array<BacklogType>>('get_backlog_types')
      .then((res) => (backlogTypes.value = res))
      .catch((e) => notifyError(e));
    invoke<Array<SnakeBacklog>>('get_backlogs', { iterationId })
      .then((res) => (model.value.priority = res.length + 1))
      .catch((e) => notifyError(e));
  });
</script>

<template>
  <NGrid :cols="12">
    <NGi :span="12">
      <NButton
        primary
        type="primary"
        size="large"
        @click="open = true"
      >
        Create
      </NButton>
    </NGi>
  </NGrid>
  <NModal v-model:show="open">
    <NCard
      title="Add a backlog"
      style="width: 600px"
    >
      <NForm
        ref="form"
        :model="model"
        size="large"
        label-placement="top"
      >
        <NGrid
          cols="6"
          x-gap="6"
        >
          <NFormItemGi
            :span="6"
            label="Title"
            :path="model.title"
            required
          >
            <NInput
              v-model:value="model.title"
              placeholder=""
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
            label="Priority"
            required
          >
            <NInputNumber
              v-model:value="model.priority"
              placeholder="EX: 1"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
            label="Hours"
            required
          >
            <NInputNumber
              v-model:value="model.hours"
              placeholder="EX: 2"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
            label="Points"
            required
          >
            <NInputNumber
              v-model:value="model.points"
              placeholder="EX: 3"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="12"
            label="Goals"
            :path="model.goals"
            required
          >
            <NInput
              v-model:value="model.goals"
              type="textarea"
              placeholder=""
            />
          </NFormItemGi>
          <NFormItemGi
            :span="12"
            label="Description"
            :path="model.description"
            required
          >
            <NInput
              v-model:value="model.description"
              type="textarea"
              placeholder=""
            />
          </NFormItemGi>
          <NFormItemGi
            :span="3"
            label="Type"
          >
            <NSelect
              v-model:value="model.type"
              :options="backlogTypes"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="3"
            label="Progress"
          >
            <NInput
              readonly
              placeholder="Undone"
            />
          </NFormItemGi>
        </NGrid>
      </NForm>
      <NSpace justify="end">
        <NButton
          size="large"
          primary
          type="primary"
          @click="debounceCreateBacklog"
          >Create</NButton
        >
      </NSpace>
    </NCard>
  </NModal>
</template>
