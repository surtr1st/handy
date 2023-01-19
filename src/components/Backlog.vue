<script setup lang="ts">
import {
  NGrid,
  NGi,
  NIcon,
  NText,
  NButton,
  NScrollbar,
  NCheckbox,
  NCard,
  NSpace,
  NStatistic,
  NDivider,
} from 'naive-ui';
import {
  CheckmarkCircle24Filled,
  ClipboardTaskListRtl24Filled,
  ArrowStepInRight12Regular,
} from '@vicons/fluent';
</script>

<template>
  <NCard
    title="[Id] Backlog Title"
    style="margin-bottom: 0.7rem"
  >
    <template #header-extra>
      <NText
        strong
        type="success"
        >100</NText
      >
    </template>
  </NCard>
  <NSpace>
    <NGrid
      :cols="12"
      :x-gap="12"
      :y-gap="12"
    >
      <NGi :span="6">
        <NCard title="Goals">
          <NScrollbar style="height: 100px">
            <NText>
              Lorem Ipsum is simply dummy text of the printing and typesetting
              industry. Lorem Ipsum has been the industry's standard dummy text
              ever since the 1500s, when an unknown printer took a galley of
              type and scrambled it to make a type specimen book.
            </NText>
          </NScrollbar>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard title="Description">
          <NScrollbar style="height: 100px">
            <NText>
              Lorem Ipsum is simply dummy text of the printing and typesetting
              industry. Lorem Ipsum has been the industry's standard dummy text
              ever since the 1500s, when an unknown printer took a galley of
              type and scrambled it to make a type specimen book. It has
              survived not only five centuries, but also the leap into
              electronic typesetting, remaining essentially unchanged. It was
              popularised in the 1960s with the release of Letraset sheets
              containing Lorem Ipsum passages, and more recently with desktop
              publishing software like Aldus PageMaker including versions of
              Lorem Ipsum.
            </NText>
          </NScrollbar>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard title="Criteria Acceptances">
          <NScrollbar style="max-height: 30vh; min-height: 15vh">
            <NSpace vertical>
              <NCheckbox v-for="i in 15">Criteria Acceptance {{ i }}</NCheckbox>
            </NSpace>
          </NScrollbar>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard title="Tasks">
          <NSpace justify="space-evenly">
            <NStatistic
              label="Total"
              value="0"
            >
              <template #prefix>
                <NIcon>
                  <ClipboardTaskListRtl24Filled />
                </NIcon>
              </template>
            </NStatistic>
            <NStatistic
              label="Done"
              value="0"
            >
              <template #prefix>
                <NIcon>
                  <CheckmarkCircle24Filled />
                </NIcon>
              </template>
            </NStatistic>
          </NSpace>
          <template #footer>
            <NDivider />
          </template>
          <template #action>
            <NSpace justify="center">
              <NButton
                type="primary"
                @click="$router.push('/iterations/:id/backlogs/:bid/tasks')"
              >
                <NIcon size="large">
                  <ArrowStepInRight12Regular />
                </NIcon>
                Show all task
              </NButton>
            </NSpace>
          </template>
        </NCard>
      </NGi>
    </NGrid>
  </NSpace>
</template>
