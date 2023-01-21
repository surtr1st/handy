<script setup lang="ts">
import { NTabs, NTabPane } from 'naive-ui';
import { defineAsyncComponent } from 'vue';

const AvailableIteration = defineAsyncComponent(
  () => import('../components/AvailableIteration.vue'),
);
const JoiningIteration = defineAsyncComponent(
  () => import('../components/JoiningIteration.vue'),
);
const FinishedIteration = defineAsyncComponent(
  () => import('../components/FinishedIteration.vue'),
);
</script>

<template>
  <NTabs
    class="card-tabs"
    default-value="all"
    size="large"
    animated
    pane-style="padding-left: 4px; padding-right: 4px; box-sizing: border-box;"
  >
    <NTabPane
      name="all"
      tab="Available"
    >
      <AvailableIteration />
    </NTabPane>
    <NTabPane
      name="current"
      tab="Current"
    >
      <JoiningIteration />
    </NTabPane>
    <NTabPane
      name="finished"
      tab="Finished"
    >
      <FinishedIteration />
    </NTabPane>
  </NTabs>
</template>
