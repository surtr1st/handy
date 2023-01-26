<script setup lang="ts">
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api';
  import { useMessages } from '../constants';
  import { useIterationRoute } from '../store';
  import { useDebounceFn } from '@vueuse/core';
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

  const open = ref(false);
  const form = ref();
  const model = ref({
    title: '',
    priority: 0,
    goals: '',
    description: '',
    hours: 0,
    points: 0,
    type: 0,
  });

  const options = [
    {
      label: 'Fixed',
      value: 1,
    },
    {
      label: 'Flexible',
      value: 2,
    },
  ];

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
      })
      .catch((message) => onError(message));
  }

  const debounceCreateBacklog = useDebounceFn(createBacklog, 300);
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
    <NGi :span="6">
      <NButton
        primary
        type="primary"
        size="large"
        @click="open = true"
      >
        Create
      </NButton>
    </NGi>
    <NGi :span="6">
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
          >
            <NInput
              v-model:value="model.title"
              placeholder=""
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
            label="Priority"
          >
            <NInputNumber
              v-model:value="model.priority"
              placeholder="EX: 1"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
            label="Hours"
          >
            <NInputNumber
              v-model:value="model.hours"
              placeholder="EX: 2"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
            label="Points"
          >
            <NInputNumber
              v-model:value="model.points"
              placeholder="EX: 3"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="12"
            label="Goals"
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
              :options="options"
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
