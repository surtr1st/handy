<script setup lang="ts">
  import { TaskListSquareLtr24Filled } from '@vicons/fluent';
  import { RefreshCircle } from '@vicons/ionicons5';
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
          value="0"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Attended"
          value="0"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Finished"
          value="0"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Created"
          value="0"
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
