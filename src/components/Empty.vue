<script setup lang="ts">
  import { NEmpty } from 'naive-ui';
  defineProps({ description: String });
</script>

<template>
  <div style="display: grid; place-items: center; min-height: 75vh">
    <NEmpty :description="description" />
  </div>
</template>
