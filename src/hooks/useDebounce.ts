export function useDebounce(fn: Function, ms = 300) {
  let timeout: ReturnType<typeof setTimeout>;
  return (self: any, ...args: any[]) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => {
      fn.apply(self, args);
    }, ms);
  };
}
