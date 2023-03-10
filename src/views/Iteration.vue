<script setup lang="ts">
  import Goals from '../components/Goals.vue';
  import BurndownChart from '../components/BurndownChart.vue';
  import BacklogCard from '../components/BacklogCard.vue';
  import BacklogCreator from '../components/BacklogCreator.vue';
  import { invoke } from '@tauri-apps/api';
  import { ref, onMounted, watch } from 'vue';
  import { RefreshCircle } from '@vicons/ionicons5';
  import { targetInvoked, useIterationRoute } from '../store';
  import { IterationStatistic, SnakeBacklog } from '../types';
  import {
    TextBulletListSquare24Filled,
    DocumentBulletList24Filled,
    DocumentCheckmark24Filled,
    DocumentDismiss24Filled,
    Pulse32Filled,
  } from '@vicons/fluent';
  import {
    NSpace,
    NScrollbar,
    NPageHeader,
    NGrid,
    NGi,
    NStatistic,
    NThing,
    NTabs,
    NTabPane,
    NButton,
    NIcon,
  } from 'naive-ui';

  const { iterationId, getIteration } = useIterationRoute();
  const backlogs = ref<Array<SnakeBacklog>>([]);
  const iterationStats = ref<IterationStatistic>({
    backlog: 0,
    backlog_done: 0,
    backlog_partially_done: 0,
    backlog_undone: 0,
    sprint_velocity: 0,
  });

  enum StatusColors {
    DONE = 'rgb(16, 185, 129)',
    PARTIALLY_DONE = 'rgb(251, 191, 36)',
    UNDONE = 'rgb(225, 29, 72)',
  }

  function fetchBacklogs() {
    invoke<Array<SnakeBacklog>>('get_backlogs', { iterationId })
      .then((res) => (backlogs.value = res))
      .catch();
  }

  function fetchIterationStats() {
    invoke<IterationStatistic>('load_stats_of_iteration', { id: iterationId })
      .then((res) => (iterationStats.value = res))
      .catch((e) => console.log(e));
  }

  watch(
    () => targetInvoked.backlogAction,
    () => fetchBacklogs(),
  );
  onMounted(() => {
    fetchBacklogs();
    fetchIterationStats();
  });
</script>

<template>
  <NPageHeader>
    <NGrid
      responsive="screen"
      :cols="5"
    >
      <NGi>
        <NStatistic
          label="Backlogs"
          v-model:value="iterationStats.backlog"
        >
          <template #prefix>
            <NIcon> <DocumentBulletList24Filled /> </NIcon>
          </template>
        </NStatistic>
      </NGi>
      <NGi>
        <NStatistic
          label="Done"
          v-model:value="iterationStats.backlog_done"
        >
          <template #prefix>
            <NIcon :color="StatusColors.DONE">
              <DocumentCheckmark24Filled />
            </NIcon>
          </template>
        </NStatistic>
      </NGi>
      <NGi>
        <NStatistic
          label="Partially Done"
          v-model:value="iterationStats.backlog_partially_done"
        >
          <template #prefix>
            <NIcon :color="StatusColors.PARTIALLY_DONE">
              <DocumentCheckmark24Filled />
            </NIcon>
          </template>
        </NStatistic>
      </NGi>
      <NGi>
        <NStatistic
          label="Undone"
          v-model:value="iterationStats.backlog_undone"
        >
          <template #prefix>
            <NIcon :color="StatusColors.UNDONE">
              <DocumentDismiss24Filled />
            </NIcon>
          </template>
        </NStatistic>
      </NGi>
      <NGi>
        <NStatistic
          label="Velocity"
          :value="iterationStats.sprint_velocity"
        >
          <template #prefix>
            <NIcon color="rgb(79, 70, 229)">
              <Pulse32Filled />
            </NIcon>
          </template>
        </NStatistic>
      </NGi>
    </NGrid>
    <template #title>
      <h2 style="text-decoration: none; color: inherit">
        [{{ iterationId }}] {{ getIteration.title }}
      </h2>
    </template>
    <template #avatar>
      <NIcon size="large">
        <TextBulletListSquare24Filled />
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
    <template #footer> </template>
  </NPageHeader>
  <NTabs
    type="segment"
    default-value="backlogs"
    animated
  >
    <NTabPane
      name="backlog-creator"
      tab="Backlog Creator"
    >
      <NGrid
        responsive="self"
        :cols="12"
        :x-gap="12"
      >
        <NGi :span="12">
          <NThing>
            <NScrollbar style="max-height: 69.5vh">
              <BacklogCreator />
            </NScrollbar>
          </NThing>
        </NGi>
      </NGrid>
    </NTabPane>
    <NTabPane
      name="backlogs"
      tab="Backlogs"
    >
      <NGrid
        responsive="self"
        :cols="12"
        :x-gap="12"
      >
        <NGi :span="3">
          <NThing>
            <NScrollbar style="max-height: 69.5vh">
              <BacklogCard
                v-for="backlog in backlogs"
                :props="{ ...backlog, list: backlogs }"
              />
            </NScrollbar>
          </NThing>
        </NGi>
        <NGi :span="9">
          <RouterView />
        </NGi>
      </NGrid>
    </NTabPane>
    <NTabPane
      name="goals"
      tab="Goals"
    >
      <NSpace
        justify="center"
        align="center"
        style="height: 50vh"
      >
        <Goals :content="getIteration.goals" />
      </NSpace>
    </NTabPane>
    <NTabPane
      name="burndown-chart"
      tab="Burndown Chart"
    >
      <NThing>
        <div style="height: 65vh; display: grid; place-items: center">
          <BurndownChart />
        </div>
      </NThing>
    </NTabPane>
  </NTabs>
</template>
