<script setup lang="ts">
  import { ref } from 'vue';
  import { NInput, NText, NCard, NButton, NSpace } from 'naive-ui';

  const switchToInput = ref<boolean>(false);
  const props = defineProps({
    content: {
      type: String,
      required: true,
    },
  });
  const edit = ref<string>(props.content);
</script>

<template>
  <NCard
    style="width: 800px"
    title="Goals"
  >
    <NText v-if="!switchToInput">{{ content }}</NText>
    <NInput
      v-else
      type="textarea"
      :resizable="false"
      v-model:value="edit"
      style="height: 200px"
    />
    <NSpace
      justify="end"
      style="margin-top: 1rem"
    >
      <NButton
        v-show="switchToInput"
        primary
        type="error"
      >
        Update
      </NButton>
      <NButton
        secondary
        type="error"
        @click="switchToInput = !switchToInput"
      >
        Change
      </NButton>
    </NSpace>
  </NCard>
</template>
