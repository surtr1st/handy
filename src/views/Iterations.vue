<script setup lang="ts">
  import { defineAsyncComponent } from 'vue';
  import { NTabs, NTabPane, NScrollbar } from 'naive-ui';

  const AvailableIteration = defineAsyncComponent(
    () => import('../components/AvailableIteration.vue'),
  );
  const JoinedIteration = defineAsyncComponent(
    () => import('../components/JoinedIteration.vue'),
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
      <NScrollbar style="height: 90vh">
        <AvailableIteration />
      </NScrollbar>
    </NTabPane>
    <NTabPane
      name="current"
      tab="Current"
    >
      <NScrollbar style="height: 90vh">
        <JoinedIteration />
      </NScrollbar>
    </NTabPane>
    <NTabPane
      name="finished"
      tab="Finished"
    >
      <NScrollbar style="height: 90vh">
        <FinishedIteration />
      </NScrollbar>
    </NTabPane>
  </NTabs>
</template>
