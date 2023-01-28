<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMounted, ref } from 'vue';
  import { RouterLink } from 'vue-router';
  import { Status20Filled } from '@vicons/fluent';
  import { useIterationRoute, backlogStore, useBacklogRoute } from '../store';
  import { colors } from '../configs/colors';
  import { ProgressOption, SnakeBacklog } from '../types';
  import { useMessages } from '../constants';
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
  const { setBacklogId } = useBacklogRoute();
  const { onError } = useMessages();

  enum StatusOptions {
    UNDONE = 1,
    PARTIALLY_DONE = 2,
    DONE = 3,
  }
  const progressOptions = ref<Array<ProgressOption>>([]);
  const option = ref<string>('');
  const statusLabel = ref<string>('Undone');
  const statusColor = ref();

  function changeStatus(value: number) {
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

  const { props } = defineProps<{
    props: SnakeBacklog & { list: Array<SnakeBacklog> };
  }>();

  function handleBacklogChosen(id: number) {
    setBacklogId(id);
    const backlog = props.list.find((bl: SnakeBacklog) => bl.id === id);
    Object.assign(backlogStore.value, backlog);
  }

  onMounted(() => {
    invoke<Array<ProgressOption>>('get_progress_options')
      .then((res) => (progressOptions.value = res))
      .catch((e) => onError(e));
  });
</script>

<template>
  <RouterLink
    :to="`/mainpage/iterations/${iid}/backlogs/${props.id}`"
    @click="handleBacklogChosen(props.id)"
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
              :options="progressOptions"
              :on-update:value="(value: number) => changeStatus(value)"
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
