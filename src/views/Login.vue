<script setup lang="ts">
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api';
  import { useRouter } from 'vue-router';
  import { useDebounceFn, useSessionStorage } from '@vueuse/core';
  import { useMessages, participant, DEBOUNCE_TIME } from '../helpers';
  import { AuthenticationResult } from '../types';
  import {
    NCard,
    NForm,
    NFormItem,
    NInput,
    NButton,
    NH1,
    NSpace,
    NText,
    FormInst,
    FormRules,
    useLoadingBar,
  } from 'naive-ui';

  const { replace } = useRouter();
  const { onError, onSuccess } = useMessages();
  const loading = useLoadingBar();

  const authButton = ref<HTMLButtonElement | null>(null);
  const form = ref<FormInst | null>(null);
  const model = ref({
    username: '',
    password: '',
  });
  const rules = ref<FormRules>({
    username: {
      required: true,
      trigger: ['blur', 'input'],
      message: 'Please input!',
    },
    password: {
      required: true,
      trigger: ['blur', 'input'],
      message: 'Please input!',
    },
  });

  function signin() {
    loading.start();
    setTimeout(() => {
      invoke<AuthenticationResult>('authenticate', {
        username: model.value.username,
        password: model.value.password,
      })
        .then((auth) => {
          const message = auth[0];
          const participantId = auth[1];
          onSuccess(message);
          useSessionStorage('PARTICIPANT_ID', participantId);
          participant.id = parseInt(
            sessionStorage.getItem('PARTICIPANT_ID') as string,
          );
          loading.finish();
          setTimeout(() => {
            replace('/mainpage/getting-started');
          }, 500);
        })
        .catch((message) => {
          loading.error();
          onError(message as string);
        });
    }, 300);
  }

  const debounceAuthentication = useDebounceFn(signin, DEBOUNCE_TIME);

  function onEnter(event: KeyboardEvent) {
    if (event.key === 'Enter') debounceAuthentication();
  }
</script>

<template>
  <NSpace
    justify="center"
    align="center"
    vertical
    style="height: 100vh; background: black"
  >
    <NCard style="width: 500px; padding: 2rem">
      <NH1>Sign In</NH1>
      <NForm
        ref="form"
        :model="model"
        :rules="rules"
      >
        <NFormItem
          path="username"
          label="Username"
        >
          <NInput
            ref="inputUsername"
            v-model:value="model.username"
            @keydown="onEnter"
            size="large"
          />
        </NFormItem>
        <NFormItem
          path="password"
          label="Password"
        >
          <NInput
            type="password"
            ref="inputPassword"
            v-model:value="model.password"
            @keydown="onEnter"
            size="large"
          />
        </NFormItem>
      </NForm>
      <NSpace
        vertical
        align="center"
        style="margin-top: 1rem"
      >
        <NText
          italic
          @click="$router.replace('/signup')"
          style="cursor: pointer"
          >No account? Sign up here!</NText
        >
        <NButton
          ref="authButton"
          primary
          type="primary"
          style="margin-top: 1rem"
          @click="debounceAuthentication"
          >Log In
        </NButton>
      </NSpace>
    </NCard>
  </NSpace>
</template>
