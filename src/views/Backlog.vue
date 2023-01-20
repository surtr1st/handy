<script setup lang="ts">
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

const { iterationId: iid } = useIterationRoute();
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
      :cols="12"
      :x-gap="12"
      :y-gap="12"
    >
      <NGi :span="6">
        <NCard
          title="Goals"
          style="width: 610px"
        >
          <NScrollbar style="height: 100px">
            <NText>
              {{ backlogStore.goals }}
            </NText>
          </NScrollbar>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard title="Description">
          <NScrollbar style="height: 100px">
            <NText>
              {{ backlogStore.description }}
            </NText>
          </NScrollbar>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard title="Criteria Acceptances">
          <NScrollbar style="max-height: 20vh; min-height: 15vh">
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
            <NSpace
              justify="center"
              align="center"
              style="height: 9vh"
            >
              <NButton
                type="primary"
                @click="
                  $router.push(
                    `/iterations/${iid}/backlogs/${backlogStore.id}/tasks`,
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
