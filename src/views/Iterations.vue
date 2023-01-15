<script setup lang="ts">
import { NThing, NSpace, NList, NListItem, NTabs, NTabPane } from 'naive-ui';
import { RouterLink } from 'vue-router';
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
            to="/iterations/:id"
            style="text-decoration: none"
          >
            <NThing
              title="Title"
              content-style="margin-top: 10px; font-size: 18px"
            >
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
            to="/iterations/:id"
            style="text-decoration: none"
          >
            <NThing
              title="Title"
              content-style="margin-top: 10px; font-size: 18px"
            >
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
