<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { ref, onMounted, onUnmounted } from 'vue';
  import { DocumentAdd24Filled } from '@vicons/fluent';
  import { useDebounceFn } from '@vueuse/core';
  import { MentionOption } from '../types';
  import {
    participant,
    useNotifications,
    useMessages,
    DEBOUNCE_TIME,
  } from '../constants';
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

  const { onError } = useMessages();
  const { notifySuccess, notifyError } = useNotifications();

  const form = ref<FormInst | null>(null);
  const range = ref<[number, number]>();
  const model = ref({
    title: '',
    goals: '',
    participants: [],
  });
  const mentions = ref<Array<MentionOption>>([]);
  const rules = ref<FormRules>({
    title: {
      required: true,
      trigger: ['blur', 'input'],
      message: 'Please input!',
    },
    goals: {
      required: true,
      trigger: ['blur', 'input'],
      message: 'Please input!',
    },
  });

  function createIteration() {
    const participants = ref<Array<number>>(model.value.participants);
    if (participants.value.length === 0) participants.value = [participant.id];
    if (participants.value.length > 1) participants.value.push(participant.id);
    const fields = {
      title: model.value.title,
      goals: model.value.goals,
      created_by: participant.id,
      participants: participants.value,
      created_date: range.value?.at(0) ?? -1,
      end_date: range.value?.at(1) ?? -1,
    };
    invoke<string>('create_iteration', { fields })
      .then((msg) => {
        notifySuccess(msg);
        model.value = {
          title: '',
          goals: '',
          participants: [],
        };
        range.value = [-1, -1];
      })
      .catch((e) => notifyError(e));
  }

  const debounceCreateIteration = useDebounceFn(createIteration, DEBOUNCE_TIME);

  onMounted(() => {
    invoke<Array<MentionOption>>('get_participants')
      .then((res) => (mentions.value = res))
      .catch((e) => onError(e));
  });
  onUnmounted(() => (mentions.value = []));
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
            v-model:value="model.title"
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
            v-model:value="model.participants"
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
            v-model:value="model.goals"
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
            @click="debounceCreateIteration"
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
