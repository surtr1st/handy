<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { SnakeIteration } from '../types';
import { useFormattedDate } from '../constants';
import {
  NThing,
  NSpace,
  NList,
  NListItem,
  NTabs,
  NTabPane,
  NDivider,
  NStatistic,
} from 'naive-ui';
import { RouterLink } from 'vue-router';
import { useIterationRoute } from '../store';

const { setIterationId } = useIterationRoute();
const iid = ref<number>(1);
const iterations = ref<Array<SnakeIteration>>([]);

onMounted(() => {
  invoke('get_iterations')
    .then((res) => (iterations.value = res as []))
    .catch((e) => console.log(e));
});

onUnmounted(() => (iterations.value = []));
</script>

<template>
  <NTabs
    class="card-tabs"
    default-value="all"
    size="large"
    animated
    pane-style="padding-left: 4px; padding-right: 4px; box-sizing: border-box;"
  >
    <NTabPane
      name="all"
      tab="All"
    >
      <NList
        hoverable
        clickable
        bordered
        style="margin-bottom: 1rem"
      >
        <NListItem
          v-for="iteration in iterations"
          :key="iteration.id"
        >
          <RouterLink
            :to="`/iterations/${iid}`"
            style="text-decoration: none"
            @click="setIterationId(iid)"
          >
            <NThing content-style="margin-top: 0px; font-size: 18px">
              <NDivider title-placement="left">{{ iteration.title }}</NDivider>
              <NSpace justify="space-around">
                <NStatistic
                  label="Id"
                  :value="`Iteration #${iteration.id}`"
                />
                <NStatistic
                  label="Current Point"
                  :value="iteration.current_point"
                />
                <NStatistic
                  label="Total Point"
                  :value="iteration.total_point"
                />
                <NStatistic
                  label="Created By"
                  :value="iteration.created_by"
                />
                <NStatistic
                  label="End Date"
                  :value="useFormattedDate(iteration?.end_date)"
                />
              </NSpace>
            </NThing>
          </RouterLink>
        </NListItem>
      </NList>
    </NTabPane>
    <NTabPane
      name="current"
      tab="Current"
    >
      <NList
        hoverable
        clickable
        bordered
        style="margin-bottom: 1rem"
      >
        <NListItem
          v-for="iteration in iterations"
          :key="iteration.id"
        >
          <RouterLink
            :to="`/iterations/${iid}`"
            style="text-decoration: none"
          >
            <NThing content-style="margin-top: 10px; font-size: 18px">
              <NDivider title-placement="left">{{ iteration.title }}</NDivider>
              <NSpace justify="space-around">
                <h4>{{ iteration.id }}</h4>
                <h4>{{ iteration.goals }}</h4>
                <h4>{{ iteration.created_by }}</h4>
                <h4>{{ iteration.end_date }}</h4>
              </NSpace>
            </NThing>
          </RouterLink>
        </NListItem>
      </NList>
    </NTabPane>
  </NTabs>
</template>
