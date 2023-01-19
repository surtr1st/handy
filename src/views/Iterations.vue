<script setup lang="ts">
import { ref } from 'vue';
import {
  NThing,
  NSpace,
  NList,
  NListItem,
  NTabs,
  NTabPane,
  NDivider,
} from 'naive-ui';
import { RouterLink } from 'vue-router';
import { useIterationRoute } from '../store';

const { setIterationId } = useIterationRoute();
const iid = ref<number>(1);
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
        <NListItem>
          <RouterLink
            :to="`/iterations/${iid}`"
            @click="setIterationId(iid)"
            style="text-decoration: none"
          >
            <NThing content-style="margin-top: 10px; font-size: 18px">
              <NDivider title-placement="left">Title</NDivider>
              <NSpace justify="space-around">
                <h4>Iteration Id</h4>
                <h4>Iteration Goals</h4>
                <h4>Participants</h4>
                <h4>End Date</h4>
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
        <NListItem>
          <RouterLink
            :to="`/iterations/${iid}`"
            style="text-decoration: none"
          >
            <NThing content-style="margin-top: 10px; font-size: 18px">
              <NDivider title-placement="left">Title</NDivider>
              <NSpace justify="space-around">
                <h4>Iteration Id</h4>
                <h4>Iteration Goals</h4>
                <h4>Participants</h4>
                <h4>End Date</h4>
              </NSpace>
            </NThing>
          </RouterLink>
        </NListItem>
      </NList>
    </NTabPane>
  </NTabs>
</template>
