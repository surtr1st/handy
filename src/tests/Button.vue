<script setup lang="ts">
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';

  const data = ref<string | unknown>('');
  function getData() {
    invoke('progress_data').then((res) => (data.value = res));
  }
</script>

<template>
  <h1>{{ data }}</h1>
  <button @click="getData()">Fetch</button>
</template>
