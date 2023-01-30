<script setup lang="ts">
  import Review from '../components/Review.vue';
  import Retrospective from '../components/Retrospective.vue';
  import { invoke } from '@tauri-apps/api';
  import { useRouter } from 'vue-router';
  import { reactive, ref, shallowRef, onMounted, onUnmounted } from 'vue';
  import { IterationKey, ReviewRetroIteration } from '../types';
  import { participant, useNotifications, useFormattedDate } from '../helpers';
  import { TextDescription24Filled } from '@vicons/fluent';
  import { retroStore, reviewStore, useIterationRoute } from '../store';
  import {
    NSelect,
    NPageHeader,
    NGrid,
    NSpace,
    NButton,
    NStatistic,
    NGi,
    NIcon,
    NDivider,
  } from 'naive-ui';

  const { replace } = useRouter();
  const { notifySuccess, notifyError } = useNotifications();
  const current = shallowRef(Review);
  const title = ref('Review Iteration');
  const iterationId = ref(0);
  const iterationKeys = ref<Array<IterationKey>>([]);
  const iteration = ref<ReviewRetroIteration>({
    end_date: 0,
    start_date: 0,
    total_point: 0,
  });

  const components = shallowRef({
    prev: Review,
    next: Retrospective,
  });

  const stepFurther = reactive({
    stepped: false,
    finish: false,
  });

  function chooseIteration(id: number) {
    invoke<ReviewRetroIteration>('get_iteration_data_when_review_retro', { id })
      .then((res) => (iteration.value = res))
      .catch((e) => notifyError(e));
    iterationId.value = id;
  }

  function onStepBack() {
    stepFurther.stepped = false;
    stepFurther.finish = false;
    current.value = components.value.prev;
    title.value = 'Review Iteration';
  }

  function onStepFurther() {
    stepFurther.stepped = true;
    stepFurther.finish = true;
    current.value = components.value.next;
    title.value = 'Retrospective';
    console.log(iteration.value);
  }

  function finish() {
    invoke<string>('end_iteration', {
      iterationId,
      reviewContent: reviewStore.content,
      retroContent: retroStore.content,
    })
      .then((message) => {
        notifySuccess(message);
        replace('/mainpage/review/retro/completed/200');
      })
      .catch((message) => notifyError(message));
  }

  onMounted(() => {
    invoke<Array<IterationKey>>('get_iteration_keys', {
      participantId: participant.id,
    })
      .then((res) => (iterationKeys.value = res))
      .catch((e) => notifyError(e));
  });
  onUnmounted(() => (iterationKeys.value = []));
</script>
<template>
  <NPageHeader subtitle="What do you feel about this iteration?">
    <NGrid
      responsive="screen"
      :cols="4"
    >
      <NGi>
        <NStatistic label="Which Iteration">
          <NSelect
            :options="iterationKeys"
            @update:value="(id: number) => chooseIteration(id)"
            style="width: 80%"
          />
        </NStatistic>
      </NGi>
      <NGi>
        <NStatistic
          label="Start Date"
          :value="useFormattedDate(iteration?.start_date as number)"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="End Date"
          :value="useFormattedDate(iteration?.end_date as number)"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Total Point"
          :value="(iteration?.total_point as number)"
        />
      </NGi>
    </NGrid>
    <template #title>
      <h2 style="text-decoration: none; color: inherit">{{ title }}</h2>
    </template>
    <template #avatar>
      <NIcon size="large">
        <TextDescription24Filled />
      </NIcon>
    </template>
    <template #extra>
      <NSpace>
        <NButton
          v-show="stepFurther.stepped"
          tertiary
          type="error"
          @click="onStepBack()"
        >
          Step Back
        </NButton>
        <NButton
          v-show="!stepFurther.finish"
          tertiary
          type="primary"
          @click="onStepFurther()"
        >
          Step Further
        </NButton>
        <NButton
          v-show="stepFurther.finish"
          tertiary
          type="success"
          @click="finish()"
        >
          Finish
        </NButton>
      </NSpace>
    </template>
    <template #footer>
      <NDivider />
    </template>
  </NPageHeader>
  <KeepAlive>
    <component :is="current" />
  </KeepAlive>
</template>
