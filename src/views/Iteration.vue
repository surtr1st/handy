<script setup lang="ts">
  import Goals from '../components/Goals.vue';
  import BurndownChart from '../components/BurndownChart.vue';
  import Backlogs from '../components/Backlogs.vue';
  import BacklogCreator from '../components/BacklogCreator.vue';
  import { invoke } from '@tauri-apps/api';
  import { ref, reactive, onMounted, watch } from 'vue';
  import { RefreshCircle } from '@vicons/ionicons5';
  import { targetInvoked, useIterationRoute } from '../store';
  import { SnakeBacklog } from '../types';
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

  const { iterationId: iid, getIteration } = useIterationRoute();
  const backlogs = ref<Array<SnakeBacklog>>([]);
  const backlogStatus = reactive({
    total: 0,
    done: 0,
    partiallyDone: 0,
    undone: 0,
  });

  enum StatusColors {
    DONE = 'rgb(16, 185, 129)',
    PARTIALLY_DONE = 'rgb(251, 191, 36)',
    UNDONE = 'rgb(225, 29, 72)',
  }

  function fetchBacklogs() {
    invoke<Array<SnakeBacklog>>('get_backlogs', { iterationId: iid })
      .then((res) => (backlogs.value = res))
      .catch();
  }

  watch(
    () => targetInvoked.backlogAction,
    () => fetchBacklogs(),
  );
  onMounted(() => fetchBacklogs());
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
          v-model:value="backlogStatus.total"
        >
          <template #prefix>
            <NIcon> <DocumentBulletList24Filled /> </NIcon>
          </template>
        </NStatistic>
      </NGi>
      <NGi>
        <NStatistic
          label="Done"
          v-model:value="backlogStatus.done"
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
          v-model:value="backlogStatus.partiallyDone"
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
          v-model:value="backlogStatus.undone"
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
          value="0"
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
        [{{ iid }}] Iteration
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
              <Backlogs
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
