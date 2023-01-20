<script setup lang="ts">
import Review from '../components/Review.vue';
import Retrospective from '../components/Retrospective.vue';
import { reactive, ref, shallowRef } from 'vue';
import { reviewStore, retroStore } from '../store';
import { useRouter } from 'vue-router';
import { TextDescription24Filled } from '@vicons/fluent';
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
  NInput,
} from 'naive-ui';

const { replace } = useRouter();
const current = shallowRef(Review);
const title = ref('Review Iteration');

const components = shallowRef({
  prev: Review,
  next: Retrospective,
});

const stepFurther = reactive({
  stepped: false,
  finish: false,
});
const options = [
  {
    label: '[id] Iteration Name',
    value: 'id',
  },
];

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
}

function finish() {
  replace('/review/retro/completed/200');
}
</script>
<template>
  <NPageHeader subtitle="What do you feel about this iteration?">
    <NGrid :cols="4">
      <NGi>
        <NStatistic label="Which Iteration">
          <NSelect
            :options="options"
            style="width: 80%"
          />
        </NStatistic>
      </NGi>
      <NGi>
        <NStatistic
          label="Start Date"
          value="11/11/2023"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="End Date"
          value="11/11/2023"
        />
      </NGi>
      <NGi>
        <NStatistic
          label="Total Point"
          value="0"
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
