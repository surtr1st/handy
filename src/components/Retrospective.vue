<script setup lang="ts">
import { QuillEditor } from '@vueup/vue-quill';
import { retroStore } from '../store';
</script>

<template>
  <QuillEditor
    v-once
    theme="bubble"
    toolbar="full"
    v-model:content="retroStore.content"
    content-type="text"
    style="height: 65vh"
  />
</template>
