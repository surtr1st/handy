<script setup lang="ts">
import { ref } from 'vue';
import { Add28Filled } from '@vicons/fluent';
import {
  NSpace,
  NModal,
  NInput,
  NGrid,
  NGi,
  NDatePicker,
  NSelect,
  NButton,
  NIcon,
  NCard,
} from 'naive-ui';

const open = ref(false);
const options = [
  {
    label: '@chi.tr',
    value: '@chi.tr',
  },
];
</script>

<template>
  <NSpace
    style="height: 85%"
    justify="center"
    align="center"
  >
    <NButton
      primary
      circle
      type="primary"
      size="large"
      @click="open = !open"
    >
      <NIcon>
        <Add28Filled />
      </NIcon>
    </NButton>
  </NSpace>
  <NModal
    :show="open"
    closable
  >
    <NCard style="width: 500px">
      <NSpace vertical>
        <NInput placeholder="Task Name" />
        <NGrid
          :cols="12"
          :x-gap="12"
        >
          <NGi :span="6">
            <NDatePicker placeholder="Created Date" />
          </NGi>
          <NGi :span="6">
            <NInput placeholder="Hour" />
          </NGi>
        </NGrid>
        <NSelect
          :options="options"
          placeholder="PIC"
        />
        <NSpace justify="center">
          <NButton
            primary
            size="large"
            style="width: 150px"
            type="primary"
          >
            Add
          </NButton>
          <NButton
            primary
            size="large"
            style="width: 150px"
            type="error"
            @click="open = false"
          >
            Cancel
          </NButton>
        </NSpace>
      </NSpace>
    </NCard>
  </NModal>
</template>
