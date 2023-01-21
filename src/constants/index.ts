import { useDateFormat } from '@vueuse/core';
import { useNotification } from 'naive-ui';

const DATE_FORMAT = 'DD-MM-YYYY';

export function useFormattedDate(date: number) {
  return useDateFormat(date, DATE_FORMAT).value;
}

export function useNotifications() {
  const { success, error } = useNotification();
  function notifySuccess(message: string, duration: number) {
    success({
      content: message,
      meta: '201',
      duration: duration,
    });
  }
  function notifyError(message: string, duration: number) {
    error({
      content: message,
      meta: '500',
      duration: duration,
    });
  }
  return { notifySuccess, notifyError };
}
