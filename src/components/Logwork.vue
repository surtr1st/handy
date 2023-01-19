<script setup lang="ts">
import dayjs from 'dayjs';
import { CSSProperties, ref } from 'vue';
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

const open = ref<boolean>(false);
const completedTime = ref<number>(new Date().getTime());

type Logwork = {
  pic: string;
  hours: number;
};
defineProps<Logwork>();

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

function logWork() {
  open.value = false;
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
        :cols="6"
        :x-gap="12"
      >
        <NGi :span="3">
          <NDatePicker
            panel
            type="date"
            v-model:value="completedTime"
          />
        </NGi>
        <NGi :span="3">
          <NSpace vertical>
            <NStatistic label="Person-In-Charge">
              <NText strong>{{ pic }}</NText>
            </NStatistic>
            <NStatistic label="Estimated Hours">
              <NText>{{ hours }}</NText>
            </NStatistic>
            <NStatistic label="Worked Time">
              <NInputNumber placeholder="Total" />
            </NStatistic>
            <NStatistic label="Completed Date">
              <NInput
                disabled
                :value="dayjs(new Date(completedTime)).format('DD-MM-YYYY')"
                placeholder=""
              />
            </NStatistic>
            <NSpace justify="end">
              <NButton
                primary
                type="primary"
                style="width: 150px"
                @click="logWork()"
              >
                Log
              </NButton>
            </NSpace>
          </NSpace>
        </NGi>
      </NGrid>
    </NCard>
  </NModal>
</template>
