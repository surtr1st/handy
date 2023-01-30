<script setup lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { ref, onMounted, onUnmounted } from 'vue';
  import { participant, useFormattedDate, useNotifications } from '../helpers';
  import { Personal, SnakeWorklog } from '../types';
  import {
    NButton,
    NForm,
    NFormItemGi,
    NGrid,
    NPageHeader,
    NDataTable,
    NInput,
    DataTableColumns,
  } from 'naive-ui';

  const { notifyError } = useNotifications();
  const worklogs = ref<Array<SnakeWorklog>>([]);
  const model = ref<Personal>({
    alias: '',
    password: '',
  });
  const edit = ref({
    alias: false,
    password: false,
  });
  const pagination = ref({
    pageSize: 7,
  });
  const columns: DataTableColumns<SnakeWorklog> = [
    {
      title: 'Task Id',
      key: 'task_id',
      sorter: (a, b) => a.task_id - b.task_id,
    },
    {
      title: 'Description',
      key: 'description',
      colSpan: (rowData, rowIndex) => (rowIndex === 2 ? 3 : 1),
    },
    {
      title: 'Worked Hours',
      key: 'worked_hours',
      sorter: (a, b) => a.worked_hours - b.worked_hours,
    },
  ];

  function fetchAliasAndPassword() {
    invoke<Personal>('get_personal_info', { id: participant.id })
      .then((res) => (model.value = res))
      .catch((e) => notifyError(e));
  }

  function fetchWorklogs() {
    invoke<Array<SnakeWorklog>>('get_worklogs', {
      participantId: participant.id,
    })
      .then((res) => (worklogs.value = res))
      .catch((e) => notifyError(e));
  }

  onMounted(() => {
    fetchAliasAndPassword();
    fetchWorklogs();
  });
  onUnmounted(() => (worklogs.value = []));
</script>

<template>
  <NPageHeader>
    <template #title>
      <h2 style="text-decoration: none; color: inherit">Participant Data</h2>
    </template>
  </NPageHeader>
  <NForm>
    <NGrid cols="12">
      <NFormItemGi
        span="7"
        label="Participant Alias"
      >
        <NInput
          :disabled="!edit.alias"
          v-model:value="model.alias"
        />
        <NButton
          v-if="!edit.alias"
          secondary
          @click="edit.alias = true"
          >Change</NButton
        >
        <NButton
          v-else
          secondary
          type="success"
          @click="edit.alias = false"
          >Update</NButton
        >
      </NFormItemGi>
      <NFormItemGi
        span="7"
        label="Password"
      >
        <NInput
          :disabled="!edit.password"
          type="password"
          v-model:value="model.alias"
        />
        <NButton
          v-if="!edit.password"
          secondary
          @click="edit.password = true"
          >Change</NButton
        >
        <NButton
          v-else
          secondary
          type="success"
          @click="edit.password = false"
          >Update</NButton
        >
      </NFormItemGi>
      <NFormItemGi
        span="12"
        label="Worklogs"
      >
        <NDataTable
          :columns="columns"
          :data="worklogs"
          :pagination="pagination"
        />
      </NFormItemGi>
    </NGrid>
  </NForm>
</template>
