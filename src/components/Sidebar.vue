<script setup lang="ts">
import { h, ref, Component } from 'vue';
import { NIcon, NMenu, NSpace, NLayout, NLayoutSider } from 'naive-ui';
import { RouterLink } from 'vue-router';
import type { MenuOption } from 'naive-ui';
import { Home, Document } from '@vicons/ionicons5';

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
    icon: renderIcon(Document),
  },
];
const activeKey = ref<string | null>(null);
const collapsed = ref(false);
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
      <NLayout>
        <RouterView />
      </NLayout>
    </NLayout>
  </NSpace>
</template>
