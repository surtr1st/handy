<script setup lang="ts">
import { h, ref, Component } from 'vue';
import {
  darkTheme,
  NConfigProvider,
  NIcon,
  NMenu,
  NSpace,
  NLayout,
  NLayoutSider,
  NNotificationProvider,
} from 'naive-ui';
import { RouterLink } from 'vue-router';
import type { MenuOption } from 'naive-ui';
import { Home } from '@vicons/ionicons5';
import {
  ClipboardTaskListRtl24Filled,
  Apps28Filled,
  PersonCircle24Filled,
} from '@vicons/fluent';
import { themeOverrides } from './configs/theme';

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: '/',
          },
        },
        { default: () => 'Getting Started' },
      ),
    key: 'getting-started',
    icon: renderIcon(Apps28Filled),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: '/home',
          },
        },
        { default: () => 'Home' },
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
            path: '/iterations',
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
            path: '/user',
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
  <NConfigProvider
    :theme="darkTheme"
    :theme-overrides="themeOverrides"
  >
    <NNotificationProvider>
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
            content-style="padding: 24px;"
            style="display: grid; place-items: center"
          >
            <RouterView />
          </NLayout>
        </NLayout>
      </NSpace>
    </NNotificationProvider>
  </NConfigProvider>
</template>
