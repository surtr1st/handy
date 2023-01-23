<script setup lang="ts">
  import { NThing, NResult, NButton } from 'naive-ui';
</script>
<template>
  <NThing>
    <NResult
      style="
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        height: 90vh;
      "
      status="success"
      title="Success"
      description="Completed Review/Retro"
    >
      <template #footer>
        <NButton @click="$router.push({ path: '/mainpage/getting-started' })"
          >Great!</NButton
        >
      </template>
    </NResult>
  </NThing>
</template>
