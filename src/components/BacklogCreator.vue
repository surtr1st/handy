<script setup lang="ts">
  import { ref } from 'vue';
  import { Archive } from '@vicons/ionicons5';
  import {
    NCard,
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

  const open = ref(false);
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
          <div
            style="
              margin-bottom: 12px;
              height: 20vh;
              display: grid;
              place-items: center;
            "
          >
            <NIcon
              size="96"
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
            Currently, only support for .CSV, .XLSX
          </NP>
        </NUploadDragger>
      </NUpload>
    </NGi>
    <NGi :span="12">
      <NButton
        primary
        type="primary"
        size="large"
        @click="open = true"
      >
        Create
      </NButton>
    </NGi>
  </NGrid>
  <NModal v-model:show="open">
    <NCard
      title="Add a backlog"
      style="width: 600px"
    >
      <NForm
        ref="form"
        :model="model"
        size="large"
        label-placement="top"
      >
        <NGrid
          cols="6"
          x-gap="6"
        >
          <NFormItemGi
            :span="6"
            label="Title"
          >
            <NInput
              v-model:value="model.title"
              placeholder=""
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
            label="Priority"
          >
            <NInputNumber
              v-model:value="model.priority"
              placeholder="EX: 1"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
            label="Hours"
          >
            <NInputNumber
              v-model:value="model.hours"
              placeholder="EX: 2"
            />
          </NFormItemGi>
          <NFormItemGi
            :span="2"
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
              placeholder=""
            />
          </NFormItemGi>
          <NFormItemGi
            :span="12"
            label="Description"
          >
            <NInput
              v-model:value="model.description"
              type="textarea"
              placeholder=""
            />
          </NFormItemGi>
        </NGrid>
      </NForm>
      <NSpace justify="end">
        <NButton
          size="large"
          primary
          type="primary"
          >Create</NButton
        >
      </NSpace>
    </NCard>
  </NModal>
</template>
