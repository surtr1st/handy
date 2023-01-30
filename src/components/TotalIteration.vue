<script setup lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { ref, onMounted } from 'vue';
  import { TaskListSquareLtr24Filled } from '@vicons/fluent';
  import { RefreshCircle } from '@vicons/ionicons5';
  import { ParticipantStatistic } from '../types';
  import { participant, useNotifications } from '../helpers';
  import {
    NPageHeader,
    NGrid,
    NSpace,
    NButton,
    NStatistic,
    NGi,
    NIcon,
    NDivider,
  } from 'naive-ui';

  const { notifyError } = useNotifications();
  const iterationStatus = ref<ParticipantStatistic>({
    current: 0,
    attended: 0,
    finished: 0,
    created: 0,
  });

  onMounted(() => {
    invoke<ParticipantStatistic>('load_stats_of_participant', {
      id: participant.id,
    })
      .then((res) => (iterationStatus.value = res))
      .catch((e) => notifyError(e));
  });
</script>

<template>
  <NPageHeader subtitle="All of the iteration action">
    <NGrid
      responsive="screen"
      :cols="5"
    >
      <NGi>
        <NStatistic
          label="Current"
          v-model:value="iterationStatus.current"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Attended"
          v-model:value="iterationStatus.attended"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Finished"
          v-model:value="iterationStatus.finished"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Created"
          v-model:value="iterationStatus.created"
        />
      </NGi>
    </NGrid>
    <template #title>
      <h2 style="text-decoration: none; color: inherit">Overview</h2>
    </template>
    <template #avatar>
      <NIcon size="large">
        <TaskListSquareLtr24Filled />
      </NIcon>
    </template>
    <template #extra>
      <NSpace>
        <NButton
          tertiary
          type="primary"
        >
          <NIcon size="large">
            <RefreshCircle />
          </NIcon>
          Refresh
        </NButton>
      </NSpace>
    </template>
    <template #footer>
      <NDivider />
    </template>
  </NPageHeader>
</template>
