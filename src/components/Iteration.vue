<script setup lang="ts">
import BurndownChart from './BurndownChart.vue';
import { h, Component } from 'vue';
import {
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
import { ChartMultiple24Filled } from '@vicons/fluent';
import { RefreshCircle } from '@vicons/ionicons5';

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) });
}
</script>

<template>
  <NPageHeader>
    <NGrid :cols="5">
      <NGi>
        <NStatistic
          label="Backlogs"
          value="0"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Done"
          value="0"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Partially Done"
          value="0"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Undone"
          value="0"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Velocity"
          value="0"
        />
      </NGi>
    </NGrid>
    <template #title>
      <h2 style="text-decoration: none; color: inherit">
        [Iteration Id] Iteration Title
      </h2>
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
  <NTabs
    type="segment"
    animated
  >
    <NTabPane
      name="backlogs"
      tab="Backlogs"
    >
      <NThing>
        <h1>Backlogs</h1>
      </NThing>
    </NTabPane>
    <NTabPane
      name="goals"
      tab="Goals"
    >
      <NThing>
        <h1>Goals</h1>
      </NThing>
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
