import { useDateFormat } from '@vueuse/core';
import { useMessage, useNotification } from 'naive-ui';

const DATE_FORMAT = 'DD-MM-YYYY';
const POPUP_DURATION = 3000;

export function useFormattedDate(date: number) {
  return useDateFormat(date, DATE_FORMAT).value;
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

export { POPUP_DURATION }
