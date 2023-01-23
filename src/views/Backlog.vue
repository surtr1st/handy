<script setup lang="ts">
  import { ref, watch } from 'vue';
  import { useIterationRoute, backlogStore } from '../store';
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
    NInput,
  } from 'naive-ui';
  import {
    CheckmarkCircle24Filled,
    ClipboardTaskListRtl24Filled,
    ArrowStepInRight12Regular,
  } from '@vicons/fluent';
  import { useWindowSize } from '@vueuse/core';

  const { height } = useWindowSize();
  const { iterationId: iid } = useIterationRoute();

  const flexHeight = ref(20);

  watch(height, (newHeight) => {
    if (newHeight > 700) flexHeight.value = 30;
    else flexHeight.value = 20;
  });
</script>

<template>
  <NCard
    :title="`[${backlogStore.id}] ${backlogStore.title}`"
    style="margin-bottom: 0.7rem"
  >
    <template #header-extra>
      <NText
        strong
        type="success"
        >{{ backlogStore.priority }}
      </NText>
    </template>
  </NCard>
  <NSpace>
    <NGrid
      responsive="self"
      item-responsive
      layout-shift-disabled
      :cols="12"
      :x-gap="12"
      :y-gap="12"
    >
      <NGi :span="6">
        <NCard title="Goals">
          <NScrollbar style="height: 100px; width: 100%">
            <NText>
              {{ backlogStore.goals }}
            </NText>
          </NScrollbar>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard title="Description">
          <NScrollbar style="height: 100px; width: 100%">
            <NText>
              {{ backlogStore.description }}
              Lorem Ipsum is simply dummy text of the printing and typesetting
              industry. Lorem Ipsum has been the industry's standard dummy text
              ever since the 1500s, when an unknown printer took a galley of
              type and scrambled it to make a type specimen book. Lorem Ipsum is
              simply dummy text of the printing and typesetting industry. Lorem
              Ipsum has been the industry's standard dummy text ever since the
              1500s, when an unknown printer took a galley of type and scrambled
              it to make a type specimen book.
            </NText>
          </NScrollbar>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard title="Criteria Acceptances">
          <NScrollbar style="height: 21.2vh; width: 100%">
            <NSpace vertical>
              <NCheckbox v-for="i in 15">Criteria Acceptance {{ i }}</NCheckbox>
            </NSpace>
          </NScrollbar>
          <template #action>
            <NInput
              type="text"
              placeholder="Enter new CA"
            />
          </template>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard
          title="Tasks"
          style="min-height: 39vh"
        >
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
            <NSpace
              justify="center"
              align="center"
            >
              <NButton
                type="primary"
                @click="
                  $router.push(
                    `/mainpage/iterations/${iid}/backlogs/${backlogStore.id}/tasks`,
                  )
                "
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
