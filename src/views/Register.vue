<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { useMessages } from '../constants';
import { useRouter } from 'vue-router';
import { useDebounceFn } from '@vueuse/core';

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
  FormItemInst,
  FormItemRule,
  useLoadingBar
} from 'naive-ui';

const loading = useLoadingBar()
const { replace } = useRouter();
const { onSuccess, onError } = useMessages();
const form = ref<FormInst | null>(null);
const rPasswordFormItemRef = ref<FormItemInst | null>(null);

interface RegisterForm {
  username: string | null;
  password: string | null;
  reEnterPassword: string | null;
}
const model = ref<RegisterForm>({
  username: null,
  password: null,
  reEnterPassword: null,
});
const rules = ref<FormRules>({
  username: {
    required: true,
    trigger: ['blur', 'input'],
    message: 'Please input!',
  },
  password: [
    {
      required: true,
      trigger: ['blur', 'input'],
      message: 'Please input!',
    },
  ],
  reEnterPassword: [
    {
      required: true,
      message: 'Re-entered password is required',
      trigger: ['input', 'blur'],
    },
    {
      validator: validatePasswordStartWith,
      message: 'Password is not same as re-entered password!',
      trigger: 'input',
    },
    {
      validator: validatePasswordSame,
      message: 'Password is not same as re-entered password!',
      trigger: ['blur', 'password-input'],
    },
  ],
});
function validatePasswordStartWith(rule: FormItemRule, value: string): boolean {
  return (
    !!model.value.password &&
    model.value.password.startsWith(value) &&
    model.value.password.length >= value.length
  );
}
function validatePasswordSame(rule: FormItemRule, value: string): boolean {
  return value === model.value.password;
}
function handlePasswordInput() {
  if (model.value.reEnterPassword) {
    rPasswordFormItemRef.value?.validate({ trigger: 'password-input' });
  }
}

function signup() {
  loading.start()
  setTimeout(() => {
    invoke('registrate', {
      username: model.value.username ?? '',
      password: model.value.password ?? '',
    })
      .then((message) => {
        loading.finish()
        onSuccess(message as string);
        setTimeout(() => {
          replace('/signin')
        }, 500)
      })
      .catch((message) => {
        loading.error()
        onError(message as string)
      });
  }, 300)
}

const handleRegistration = useDebounceFn(signup)
</script>

<template>
  <NSpace justify="center" align="center" vertical style="height: 100vh; background: black">
    <NCard style="width: 500px; padding: 2rem">
      <NH1>Sign Up</NH1>
      <NForm ref="form" :model="model" :rules="rules">
        <NFormItem path="username" label="Username">
          <NInput v-model:value="model.username" size="large" />
        </NFormItem>
        <NFormItem path="password" label="Password">
          <NInput type="password" v-model:value="model.password" size="large" @input="handlePasswordInput" />
        </NFormItem>
        <NFormItem path="reEnterPassword" label="Re-enter Password" ref="rPasswordFormItemRef" first>
          <NInput type="password" v-model:value="model.reEnterPassword" size="large" :disabled="!model.password" />
        </NFormItem>
      </NForm>
      <NSpace vertical align="center" style="margin-top: 1rem">
        <NText italic @click="$router.replace('/signin')" style="cursor: pointer">Already have an account? Sign in here!
        </NText>
        <NButton primary :disabled="model.reEnterPassword !== model.password" type="primary" style="margin-top: 1rem"
          @click="handleRegistration">Registrate</NButton>
      </NSpace>
    </NCard>
  </NSpace>
</template>
