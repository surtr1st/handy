<script setup lang="ts">
  import { ref } from 'vue';
  import { Archive } from '@vicons/ionicons5';
  import {
    NModal,
    NGrid,
    NGi,
    NForm,
    NFormItemGi,
    NInput,
    NInputNumber,
    NUpload,
    NUploadDragger,
    NP,
    NIcon,
    NText,
    NSpace,
    NButton,
  } from 'naive-ui';

  const form = ref();
  const model = ref({
    title: '',
    priority: 0,
    goals: '',
    description: '',
    hours: 0,
    points: 0,
    iterationId: 0,
  });
</script>

<template>
  <NGrid :cols="12">
    <NGi :span="12">
      <NUpload>
        <NUploadDragger>
          <div style="margin-bottom: 12px">
            <NIcon
              size="48"
              :depth="3"
            >
              <Archive />
            </NIcon>
          </div>
          <NText style="font-size: 16px">
            Click or drag a file to this area to upload
          </NText>
          <NP
            depth="3"
            style="margin: 8px 0 0 0"
          >
            Strictly prohibit from uploading sensitive information. For example,
            your bank card PIN or your credit card expiry date.
          </NP>
        </NUploadDragger>
      </NUpload>
    </NGi>
    <NGi :span="12">
      <NButton> Create </NButton>
    </NGi>
  </NGrid>

  <NModal>
    <NForm
      ref="form"
      :model="model"
      size="large"
      label-placement="top"
    >
      <NFormItemGi
        :span="4"
        label="Title"
      >
        <NInput
          v-model:value="model.title"
          placeholder="Title"
        />
      </NFormItemGi>
      <NFormItemGi
        :span="4"
        label="Priority"
      >
        <NInputNumber
          v-model:value="model.priority"
          placeholder="EX: 1"
        />
      </NFormItemGi>
      <NFormItemGi
        :span="4"
        label="Hours"
      >
        <NInputNumber
          v-model:value="model.hours"
          placeholder="EX: 2"
        />
      </NFormItemGi>
      <NFormItemGi
        :span="4"
        label="Points"
      >
        <NInputNumber
          v-model:value="model.points"
          placeholder="EX: 3"
        />
      </NFormItemGi>
      <NFormItemGi
        :span="12"
        label="Goals"
      >
        <NInput
          v-model:value="model.goals"
          type="textarea"
        />
      </NFormItemGi>
      <NFormItemGi
        :span="12"
        label="Description"
      >
        <NInput
          v-model:value="model.description"
          type="textarea"
        />
      </NFormItemGi>
      <NFormItemGi>
        <NSpace>
          <NButton>Create</NButton>
        </NSpace>
      </NFormItemGi>
    </NForm>
  </NModal>
</template>
