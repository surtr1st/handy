<script setup lang="ts">
import { NEmpty, NButton } from 'naive-ui';
</script>

<template>
  <div style="display: grid; place-items: center; min-height: 90vh">
    <NEmpty
      description="Welcome to Handy! The Iteration Agile Management Tool."
    >
      <template #extra>
        <NButton
          size="large"
          type="primary"
          @click="$router.replace('/home')"
        >
          Create Iteration
        </NButton>
      </template>
    </NEmpty>
  </div>
</template>
