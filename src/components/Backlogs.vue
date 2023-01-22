<script setup lang="ts">
import { ref } from 'vue';
import { RouterLink } from 'vue-router';
import { Status20Filled } from '@vicons/fluent';
import { useIterationRoute, backlogStore } from '../store';
import { colors } from '../configs/colors';
import { Backlog } from '../types';
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

const { iterationId: iid } = useIterationRoute();
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

function calculatePointPercentage(workHours: number, point: number) {
  return Math.round(workHours * 100) / 100;
}

const { props } = defineProps<{ props: Backlog & { list: Array<Backlog> } }>();

function setBacklog(id: number) {
  const backlog = props.list.find((bl: Backlog) => bl.id === id);
  backlogStore.id = backlog?.id as number;
  backlogStore.title = backlog?.title as string;
  backlogStore.goals = backlog?.goals as string;
  backlogStore.description = backlog?.description as string;
  backlogStore.priority = backlog?.priority as number;
  backlogStore.hours = backlog?.hours as number;
  backlogStore.points = backlog?.points as number;
  backlogStore.createdDate = backlog?.createdDate as number;
}
</script>

<template>
  <RouterLink
    :to="`/mainpage/iterations/${iid}/backlogs/${props.id}`"
    @click="setBacklog(props.id)"
    style="text-decoration: none"
  >
    <NCard
      :title="props.title"
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
          {{ props.priority }}
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
            <template #suffix>/ {{ props.hours }}</template>
          </NStatistic>
          <NStatistic label="Points">
            <template #prefix>
              <NProgress
                type="circle"
                :color="colors.primary"
                :percentage="
                  calculatePointPercentage(props.hours, props.points)
                "
                style="width: 40px"
              >
                <span style="font-size: 12px">{{ props.points }}</span>
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
