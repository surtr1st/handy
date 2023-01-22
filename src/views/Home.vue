<script setup lang="ts">
import { h, ref, Component } from 'vue';
import { NIcon, NMenu, NSpace, NLayout, NLayoutSider } from 'naive-ui';
import { RouterLink } from 'vue-router';
import type { MenuOption } from 'naive-ui';
import { Home } from '@vicons/ionicons5';
import {
  ClipboardTaskListRtl24Filled,
  Apps28Filled,
  PersonCircle24Filled,
  NotepadPerson24Filled,
} from '@vicons/fluent';

function renderIcon(icon: Component) {
  return () => h(NIcon, { size: 'large' }, { default: () => h(icon) });
}

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: '/mainpage/getting-started',
          },
        },
        { default: () => 'Getting Started' },
      ),
    key: 'home',
    icon: renderIcon(Home),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: '/mainpage/create-iteration',
          },
        },
        { default: () => 'Create a Iteration' },
      ),
    key: 'create-iteration',
    icon: renderIcon(Apps28Filled),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: '/mainpage/iterations',
          },
        },

        { default: () => 'Iterations' },
      ),
    key: 'iterations',
    icon: renderIcon(ClipboardTaskListRtl24Filled),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: '/mainpage/end-of-iteration',
          },
        },

        { default: () => 'Review/Retro' },
      ),
    key: 'review',
    icon: renderIcon(NotepadPerson24Filled),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: '/mainpage/me',
          },
        },

        { default: () => 'Me' },
      ),
    key: 'user',
    icon: renderIcon(PersonCircle24Filled),
  },
];
const activeKey = ref<string | null>(null);
const collapsed = ref(true);
</script>

<template>
  <NSpace vertical>
    <NLayout
      has-sider
      style="height: 100vh"
    >
      <NLayoutSider
        bordered
        collapse-mode="width"
        :collapsed-width="64"
        :width="240"
        :collapsed="collapsed"
        show-trigger
        @collapse="collapsed = true"
        @expand="collapsed = false"
      >
        <NMenu
          :icon-size="30"
          v-model:value="activeKey"
          :collapsed="collapsed"
          :collapsed-width="64"
          :collapsed-icon-size="22"
          :options="menuOptions"
        />
      </NLayoutSider>
      <NLayout
        :native-scrollbar="false"
        content-style="padding: 1rem;"
        style="display: grid; place-items: center"
      >
        <RouterView />
      </NLayout>
    </NLayout>
  </NSpace>
</template>
