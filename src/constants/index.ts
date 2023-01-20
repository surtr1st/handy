import { useDateFormat } from '@vueuse/core';

const DATE_FORMAT = 'DD-MM-YYYY';

export function useFormattedDate(date: number) {
  return useDateFormat(date, DATE_FORMAT).value;
}
