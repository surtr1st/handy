<script setup lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { useDebounceFn } from '@vueuse/core';
  import { onMounted, onUnmounted, ref, watch } from 'vue';
  import {
    useIterationRoute,
    useBacklogRoute,
    backlogStore,
    targetInvoked,
  } from '../store';
  import { CriteriaAcceptance } from '../types';
  import { DEBOUNCE_TIME, useMessages, useNotifications } from '../constants';
  import {
    CheckmarkCircle24Filled,
    ClipboardTaskListRtl24Filled,
    ArrowStepInRight12Regular,
  } from '@vicons/fluent';
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

  const { iterationId: iid } = useIterationRoute();
  const { backlogId } = useBacklogRoute();
  const { notifyError } = useNotifications();
  const { onSuccess, onError } = useMessages();

  const cas = ref<Array<CriteriaAcceptance>>([]);
  const caTitle = ref<string>('');

  function addCriteriaAcceptance() {
    const fields = {
      title: caTitle.value,
      status: false,
      backlog_id: backlogId,
    };
    invoke<string>('create_criteria_acceptance', { fields })
      .then((message) => {
        onSuccess(message);
        targetInvoked.criteriaAcceptanceAction =
          !targetInvoked.criteriaAcceptanceAction;
        caTitle.value = '';
      })
      .catch((message) => onError(message));
  }

  function markAsDoneCriteria(id: number, value: boolean) {
    console.log(id, value);
    invoke<string>('update_criteria_acceptance', {
      id,
      backlogId,
      value,
    })
      .then(
        () =>
          (targetInvoked.criteriaAcceptanceAction =
            !targetInvoked.criteriaAcceptanceAction),
      )
      .catch((message) => onError(message));
  }

  function removeCriteriaAcceptance(id: number) {
    invoke<string>('remove_criteria_acceptance', { id, backlogId })
      .then(
        () =>
          (targetInvoked.criteriaAcceptanceAction =
            !targetInvoked.criteriaAcceptanceAction),
      )
      .catch((message) => onError(message));
  }

  const debounceAddCriteriaAcceptance = useDebounceFn(
    addCriteriaAcceptance,
    DEBOUNCE_TIME,
  );

  const debounceUpdateCriteriaAcceptance = useDebounceFn(
    markAsDoneCriteria,
    DEBOUNCE_TIME,
  );

  function fetchCriteriaAcceptances() {
    invoke<Array<CriteriaAcceptance>>('get_criteria_acceptances', {
      backlogId,
    })
      .then((res) => (cas.value = res))
      .catch((e) => notifyError(e));
  }

  function onEnter(event: KeyboardEvent) {
    if (event.key === 'Enter') debounceAddCriteriaAcceptance();
  }

  watch(
    () => targetInvoked.criteriaAcceptanceAction,
    () => fetchCriteriaAcceptances(),
  );
  onMounted(() => fetchCriteriaAcceptances());
  onUnmounted(() => (cas.value = []));
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
          <NScrollbar style="height: 100px; width: 100vw">
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
            </NText>
          </NScrollbar>
        </NCard>
      </NGi>
      <NGi :span="6">
        <NCard title="Criteria Acceptances">
          <NScrollbar style="height: 21.2vh; width: 100%">
            <NSpace vertical>
              <NSpace
                justify="space-between"
                align="center"
                v-for="ca in cas"
                :key="ca.id"
              >
                <NCheckbox
                  :value="ca.id"
                  :checked="ca.status"
                  @update:checked="(value: boolean) => debounceUpdateCriteriaAcceptance(ca.id, value)"
                >
                  {{ ca.title }}
                </NCheckbox>
                <NButton
                  secondary
                  type="error"
                  @click="removeCriteriaAcceptance(ca.id)"
                  >Remove</NButton
                >
              </NSpace>
            </NSpace>
          </NScrollbar>
          <template #action>
            <NInput
              type="text"
              v-model:value="caTitle"
              placeholder="Enter new CA"
              @keydown="onEnter"
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
