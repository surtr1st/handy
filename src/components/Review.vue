<script setup lang="ts">
  import { QuillEditor } from '@vueup/vue-quill';
  import { reviewStore } from '../store';
</script>

<template>
  <QuillEditor
    v-once
    theme="bubble"
    toolbar="full"
    v-model:content="reviewStore.content"
    content-type="text"
    style="height: 65vh"
  />
</template>
