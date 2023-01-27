<script setup lang="ts">
import { RouterView } from 'vue-router';
import {
  darkTheme,
  NConfigProvider,
  NNotificationProvider,
  NMessageProvider,
  NLoadingBarProvider,
} from 'naive-ui';
import { themeOverrides } from './configs/theme';
</script>

<template>
  <NConfigProvider :theme="darkTheme" :theme-overrides="themeOverrides">
    <NLoadingBarProvider>
      <NMessageProvider>
        <NNotificationProvider>
          <RouterView />
        </NNotificationProvider>
      </NMessageProvider>
    </NLoadingBarProvider>
  </NConfigProvider>
</template>
