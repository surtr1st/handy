<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { DocumentAdd24Filled } from '@vicons/fluent';
import { useDebounceFn } from '@vueuse/core';
import { useNotifications } from '../constants';
import {
  NIcon,
  NButton,
  NCard,
  NInput,
  NSelect,
  NDatePicker,
  NForm,
  NGrid,
  NFormItemGi,
  FormInst,
  FormRules,
} from 'naive-ui';

// Variables
const duration = 3000;
const { notifySuccess, notifyError } = useNotifications();

const form = ref<FormInst | null>(null);
const range = ref<[number, number]>();
const model = ref({
  input: '',
  textarea: '',
  select: [],
});
const mentions = [
  {
    label: 'chi.tr',
    value: 'chi.tr',
  },
];
const rules = ref<FormRules>({
  input: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Please input!',
  },
  textarea: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Please input!',
  },
});

// Functionality

function createIteration() {
  const iteration = {
    title: model.value.input,
    goals: model.value.textarea,
    createdBy: model.value.select[0] ?? '',
    createdDate: range.value?.at(0) ?? -1,
    endDate: range.value?.at(1) ?? -1,
  };
  invoke('create_iteration', { ...iteration })
    .then((msg) => {
      notifySuccess(msg as string, duration);
      model.value = {
        input: '',
        textarea: '',
        select: [],
      };
      range.value = [-1, -1];
    })
    .catch((e) => notifyError(e as string, duration));
}

const debounce = useDebounceFn(createIteration, 300);
</script>

<template>
  <NCard
    title="Create a Iteration"
    :segmented="{
      content: true,
      footer: 'soft',
    }"
  >
    <NForm
      ref="form"
      :model="model"
      :rules="rules"
      label-placement="top"
      size="large"
    >
      <NGrid
        responsive="screen"
        x-gap="12"
        y-gap="12"
        cols="4"
      >
        <NFormItemGi
          span="2"
          label="Iteration Title"
          path="input"
          size="large"
        >
          <NInput
            v-model:value="model.input"
            placeholder="Title"
            clearable
          />
        </NFormItemGi>
        <NFormItemGi
          span="2"
          label="Participants"
          size="large"
          path="select"
        >
          <NSelect
            v-model:value="model.select"
            multiple
            placeholder="Who will join this iteration"
            :options="mentions"
          />
        </NFormItemGi>
        <NFormItemGi
          span="4"
          label="Timeline"
          size="large"
        >
          <NDatePicker
            v-model:value="range"
            type="daterange"
            size="large"
            clearable
            style="width: 100%"
          />
        </NFormItemGi>
        <NFormItemGi
          span="10"
          label="Goals"
          path="textarea"
          size="large"
        >
          <NInput
            v-model:value="model.textarea"
            placeholder="Description"
            type="textarea"
            :autosize="{
              minRows: 3,
              maxRows: 5,
            }"
          />
        </NFormItemGi>
        <NFormItemGi span="10">
          <NButton
            primary
            type="primary"
            @click="debounce"
          >
            <template #icon>
              <NIcon size="20">
                <DocumentAdd24Filled />
              </NIcon>
            </template>
            Create
          </NButton>
        </NFormItemGi>
      </NGrid>
    </NForm>
  </NCard>
</template>
