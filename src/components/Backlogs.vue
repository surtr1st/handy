<script setup lang="ts">
import { colors } from '../configs/colors';
import { RouterLink } from 'vue-router';
import { ref } from 'vue';
import {
  NBadge,
  NPopselect,
  NButton,
  NText,
  NCard,
  NStatistic,
  NSpace,
  NProgress,
  NIcon,
} from 'naive-ui';
import { Status20Filled } from '@vicons/fluent';
import { useIterationRoute } from '../store';

const { iterationId } = useIterationRoute();
const id = 1;
enum StatusOptions {
  DONE = 'done',
  PARTIALLY_DONE = 'partially_done',
  UNDONE = 'undone',
}

const options = [
  {
    label: 'Done',
    value: 'done',
  },
  {
    label: 'Partially Done',
    value: 'partially_done',
  },
  {
    label: 'Undone',
    value: 'undone',
  },
];

const option = ref<string>('');
const statusLabel = ref<string>('Undone');
const statusColor = ref();

function changeStatus(value: string) {
  switch (value) {
    case StatusOptions.DONE:
      statusColor.value = 'success';
      statusLabel.value = 'Done';
      break;
    case StatusOptions.PARTIALLY_DONE:
      statusColor.value = 'warning';
      statusLabel.value = 'Partially Done';
      break;
    default:
      statusColor.value = 'error';
      statusLabel.value = 'Undone';
      break;
  }
}
</script>

<template>
  <RouterLink
    :to="`/iterations/${iterationId}/backlogs/${id}`"
    style="text-decoration: none"
  >
    <NCard
      title="Title"
      hoverable
      style="margin-bottom: 0.7rem"
    >
      <template #header-extra>
        <NBadge
          :value="statusLabel"
          :type="statusColor ?? 'error'"
          style="margin-right: 0.5rem"
        />
        <NText
          type="success"
          strong
        >
          100
        </NText>
      </template>
      <template #action>
        <NSpace justify="space-evenly">
          <NStatistic
            label="Tasks"
            value="0"
          >
            <template #suffix>/ 0</template>
          </NStatistic>
          <NStatistic
            label="Hours"
            value="0"
          >
            <template #suffix>/ 0</template>
          </NStatistic>
          <NStatistic label="Points">
            <template #prefix>
              <NProgress
                type="circle"
                :color="colors.primary"
                :percentage="70"
                style="width: 40px"
              >
                <span>7</span>
              </NProgress>
            </template>
          </NStatistic>
          <NStatistic label="Status">
            <NPopselect
              v-model:value="option"
              :options="options"
              :on-update:value="(value: string) => changeStatus(value)"
            >
              <NButton>
                <NIcon>
                  <Status20Filled />
                </NIcon>
              </NButton>
            </NPopselect>
          </NStatistic>
        </NSpace>
      </template>
    </NCard>
  </RouterLink>
</template>
