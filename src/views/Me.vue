<script setup lang="ts">
  import { ref } from 'vue';
  import { SnakeWorklog } from '../types';
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

  const worklogs = ref<Array<SnakeWorklog>>([]);
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
      key: 'task-id',
      sorter: (a, b) => a.task_id - b.task_id,
    },
    {
      title: 'Completed Date',
      key: 'completed-date',
      sorter: (a, b) => a.completed_date - b.completed_date,
    },
    {
      title: 'Description',
      key: 'desc',
      colSpan: (rowData, rowIndex) => (rowIndex === 2 ? 3 : 2),
    },
    {
      title: 'Worked Hours',
      key: 'worked-hours',
      sorter: (a, b) => a.hour - b.hour,
    },
  ];
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
          value="@"
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
