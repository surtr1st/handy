<script setup lang="ts">
import { defineAsyncComponent, h, onMounted, onUnmounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { SnakeIteration } from '../types';
import { RouterLink } from 'vue-router';
import { participant, useFormattedDate } from '../constants';
import { RecycleScroller } from 'vue-virtual-scroller';
import { useIterationRoute } from '../store';
import {
  NList,
  NListItem,
  NThing,
  NText,
  NSpace,
  NStatistic,
} from 'naive-ui';

const { setIterationId, setIteration } = useIterationRoute();
const Empty = defineAsyncComponent(() => import('./Empty.vue'));
const iterations = ref<Array<SnakeIteration>>([]);

function handleIterationChosen(id: number) {
  const iteration = iterations.value.find((i: SnakeIteration) => i.id === id);
  setIterationId(id);
  setIteration(iteration as SnakeIteration);
}

onMounted(() => {
  invoke<Array<SnakeIteration>>('get_joined_iterations', {
    participantId: participant.id,
  })
    .then((res) => (iterations.value = res))
    .catch((e) => console.log(e));
});
onUnmounted(() => (iterations.value = []));
</script>

<template>
  <Empty v-if="iterations.length < 1" description="You haven't joined any iteration!" />
  <RecycleScroller v-else class="scroller" :items="iterations" v-slot="{ item }" :item-size="129" :list-tag="
    h(NList, {
      hoverable: true,
      clickable: true,
      bordered: true,
    })
  " :item-tag="NListItem">
    <RouterLink :to="`/mainpage/iterations/${item.id}`" style="text-decoration: none"
      @click="handleIterationChosen(item.id)">
      <NThing content-style="font-size: 18px">
        <template #header>
          <NText strong>{{ item.title }}</NText>
        </template>
        <NSpace justify="space-around">
          <NStatistic label="Id" :value="`#${item.id}`" />
          <NStatistic label="Current Point" :value="item.current_point" />
          <NStatistic label="Total Point" :value="item.total_point" />
          <NStatistic label="Created By" :value="item.created_by" />
          <NStatistic label="End Date" :value="useFormattedDate(item?.end_date)" />
        </NSpace>
      </NThing>
    </RouterLink>
  </RecycleScroller>
</template>
