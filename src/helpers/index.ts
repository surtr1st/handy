import { useDateFormat } from '@vueuse/core';
import { useMessage, useNotification } from 'naive-ui';
import { reactive } from 'vue';

const DEBOUNCE_TIME = 300;
const DATE_FORMAT_DMY = 'DD-MM-YYYY';
const DATE_FORMAT_YMD = 'YYYY-MM-DD';
const POPUP_DURATION = 3000;
const participant = reactive({
  id: parseInt(sessionStorage.getItem('PARTICIPANT_ID') as string),
});

export function useFormattedDate(date: number, format?: string) {
  return useDateFormat(date, format ?? DATE_FORMAT_DMY).value;
}

export function useNotifications() {
  const { success, error } = useNotification();
  function notifySuccess(message: string, duration?: number) {
    success({
      content: message,
      meta: '201',
      duration: duration ?? POPUP_DURATION,
    });
  }
  function notifyError(message: string, duration?: number) {
    error({
      content: message,
      meta: '500',
      duration: duration ?? POPUP_DURATION,
    });
  }
  return { notifySuccess, notifyError };
}

export function useMessages() {
  const { success, error } = useMessage();
  const onSuccess = (message: string, duration?: number) =>
    success(message, { duration: duration ?? POPUP_DURATION });
  const onError = (message: string, duration?: number) =>
    error(message, { duration: duration ?? POPUP_DURATION });
  return { onSuccess, onError };
}

export {
  DEBOUNCE_TIME,
  POPUP_DURATION,
  participant,
  DATE_FORMAT_DMY,
  DATE_FORMAT_YMD,
};
