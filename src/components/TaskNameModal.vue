<script setup lang="ts">
import { NModal, NCard, NInput } from 'naive-ui';
type TaskNameModal = {
  value: string;
  closeOnEnter: (event: KeyboardEvent) => void;
};
defineProps<TaskNameModal>();
</script>
<template>
  <NModal v-model:show="">
    <NCard
      style="width: 600px"
      title="Edit Name"
      role="dialog"
    >
      <NInput
        :value="value"
        @keydown="closeOnEnter"
      />
    </NCard>
  </NModal>
</template>
