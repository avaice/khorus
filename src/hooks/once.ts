export function once<T>(load: () => Promise<T>) {
  let promise: Promise<T> | undefined;
  return () => (promise ??= load());
}
