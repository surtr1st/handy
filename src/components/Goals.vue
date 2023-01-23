<script setup lang="ts">
  import { ref } from 'vue';
  import { NInput, NText, NCard, NButton, NSpace } from 'naive-ui';

  const switchToInput = ref<boolean>(false);

  const content = `
      Lorem Ipsum is simply dummy text of the printing and typesetting industry.
      Lorem Ipsum has been the industry's standard dummy text ever since the
      1500s, when an unknown printer took a galley of type and scrambled it to
      make a type specimen book. Lorem Ipsum is simply dummy text of the
      printing and typesetting industry. Lorem Ipsum has been the industry's
      standard dummy text ever since the 1500s, when an unknown printer took a
      galley of type and scrambled it to make a type specimen book.`;
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
      :value="content"
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
