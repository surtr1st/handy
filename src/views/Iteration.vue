<script setup lang="ts">
import Goals from '../components/Goals.vue';
import BurndownChart from '../components/BurndownChart.vue';
import Backlogs from '../components/Backlogs.vue';
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
import {
  TextBulletListSquare24Filled,
  DocumentBulletList24Filled,
  DocumentCheckmark24Filled,
  DocumentDismiss24Filled,
  Pulse32Filled,
} from '@vicons/fluent';
import { RefreshCircle } from '@vicons/ionicons5';
import { useIterationRoute } from '../store';
const { iterationId } = useIterationRoute();

enum StatusColors {
  DONE = 'rgb(16, 185, 129)',
  PARTIALLY_DONE = 'rgb(251, 191, 36)',
  UNDONE = 'rgb(225, 29, 72)',
}
</script>

<template>
  <NPageHeader>
    <NGrid :cols="5">
      <NGi>
        <NStatistic
          label="Backlogs"
          value="0"
        >
          <template #prefix>
            <NIcon> <DocumentBulletList24Filled /> </NIcon>
          </template>
        </NStatistic>
      </NGi>
      <NGi>
        <NStatistic
          label="Done"
          value="0"
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
          value="0"
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
          value="0"
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
        [{{ iterationId }}] Iteration
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
      <NGrid
        :cols="12"
        :x-gap="12"
      >
        <NGi :span="3">
          <NThing>
            <NScrollbar style="max-height: 71vh">
              <Backlogs v-for="i in 5" />
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
        <Goals />
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