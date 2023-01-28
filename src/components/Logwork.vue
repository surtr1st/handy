<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { CSSProperties, ref } from 'vue';
  import { Logwork, SnakeLogwork } from '../types';
  import {
    participant,
    useFormattedDate,
    useNotifications,
  } from '../constants';
  import {
    NGrid,
    NGi,
    NSpace,
    NStatistic,
    NInput,
    NInputNumber,
    NText,
    NThing,
    NSwitch,
    NModal,
    NCard,
    NDatePicker,
    NButton,
  } from 'naive-ui';

  const { notifySuccess, notifyError } = useNotifications();
  const open = ref<boolean>(false);
  const description = ref<string>('');
  const workedHours = ref<number>(0);
  defineProps<{ props: Partial<Logwork> }>();

  const railStyle = ({ checked }: { checked: boolean }) => {
    const style: CSSProperties = {};
    style.fontWeight = 'bolder';
    if (checked) {
      style.background = 'rgb(16, 185, 129)';
    } else {
      style.background = 'rgb(225, 29, 72)';
    }
    return style;
  };

  function logWork(taskId: number, completedDate: number) {
    const fields = {
      completed_time: completedDate,
      worked_hours: workedHours.value,
      task_id: taskId,
      participant_id: participant.id,
    };
    invoke<string>('log_work', { taskId, fields })
      .then((message) => {
        notifySuccess(message);
        open.value = false;
      })
      .catch((message) => notifyError(message));
  }
</script>

<template>
  <NThing @click="open = true">
    <NSwitch
      size="large"
      :rail-style="railStyle"
      :value="open"
    >
      <template #checked> Done </template>
      <template #unchecked> Undone </template>
    </NSwitch>
  </NThing>
  <NModal v-model:show="open">
    <NCard
      title="Logwork"
      style="width: 650px"
    >
      <NGrid
        responsive="self"
        :cols="6"
        :x-gap="12"
      >
        <NGi :span="3">
          <NDatePicker
            panel
            type="date"
            v-model:value="props.startedDate"
          />
        </NGi>
        <NGi :span="3">
          <NSpace vertical>
            <NStatistic label="Person-In-Charge">
              <NText strong>{{ props.pic }}</NText>
            </NStatistic>
            <NStatistic label="Estimated Hours">
              <NText>{{ props.estimatedHours }}</NText>
            </NStatistic>
            <NStatistic label="Worked Time">
              <NInputNumber
                placeholder="Total"
                v-model:value="workedHours"
              />
            </NStatistic>
            <NStatistic label="Completed Date">
              <NInput
                disabled
                :value="useFormattedDate(props.startedDate as number)"
              />
            </NStatistic>
          </NSpace>
        </NGi>
        <NGi :span="6">
          <NStatistic label="Description">
            <NInput
              type="textarea"
              v-model:value="description"
              placeholder=""
            />
          </NStatistic>
          <NSpace
            justify="end"
            align="center"
          >
            <NText
              v-if="props.estimatedHours === 0"
              type="error"
              >You cannot logwork now. Your estimate hours haven't set!</NText
            >
            <NButton
              primary
              type="primary"
              style="width: 150px"
              :disabled="props.estimatedHours === 0"
              @click="
                logWork(props.taskId as number, props.startedDate as number)
              "
            >
              Log
            </NButton>
          </NSpace>
        </NGi>
      </NGrid>
    </NCard>
  </NModal>
</template>
