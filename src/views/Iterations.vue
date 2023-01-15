<script setup lang="ts">
import { h } from 'vue';
import { NMenu, NIcon } from 'naive-ui';
import type { MenuOption } from 'naive-ui';
import { RouterLink } from 'vue-router';

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        'a',
        {
          href: '/iterations/i',
        },
        {
          default: () => 'A du',
        },
      ),
    key: 'iteration-1',
  },
];
</script>

<template>
  <NMenu :options="menuOptions" />
</template>
