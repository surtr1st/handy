<script setup lang="ts">
import { ref } from 'vue';
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
import { DocumentAdd24Filled } from '@vicons/fluent';

const form = ref<FormInst | null>(null);
const model = ref({
  input: null,
  textarea: null,
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
  select: {
    required: true,
    trigger: ['change'],
    message: 'Please select who will joining this iteration!',
    validator(rule: unknown, value: string[]) {
      if (value.length >= 5) return new Error('Up to 4 tags');
      return true;
    },
  },
});
const range = ref<[number, number]>();
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
            type="primary"
            color="#6719ff"
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
